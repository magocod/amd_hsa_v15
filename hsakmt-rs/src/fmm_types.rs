#![allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    non_upper_case_globals,
    clippy::enum_clike_unportable_variant,
    clippy::mixed_case_hex_literals
)]

use crate::fmm_types::svm_aperture_type::SVM_DEFAULT;
use crate::hsakmttypes::{HsaMemFlagUnion, HsaMemFlags, HSA_ENGINE_ID};
use crate::rbtree::{rbtree_node_t, rbtree_s, rbtree_t};
use amdgpu_drm_sys::bindings::amdgpu_device;

#[derive(Debug, Clone)]
pub struct vm_area<'a> {
    pub start: *mut std::os::raw::c_void,
    pub end: *mut std::os::raw::c_void,
    pub next: Option<&'a vm_area<'a>>,
    pub prev: Option<&'a vm_area<'a>>,
}

pub type vm_area_t<'a> = vm_area<'a>;

pub struct HsakmtGlobalsArgs {
    pub page_size: i32,
    pub fmm_svm_alignment_order: u32,
}

/* Aperture management function pointers to allow different management
 * schemes.
 */
#[allow(clippy::type_complexity)]
#[derive(Debug, Clone)]
pub struct manageable_aperture_ops_t {
    // allocate_area_aligned: &'static fn(&[u8]) -> *mut std::os::raw::c_void,
    pub allocate_area_aligned: Option<
        unsafe fn(
            aper: &manageable_aperture_t,
            addr: *mut std::os::raw::c_void,
            size: u64,
            align: u64,
            hsakmt_global: HsakmtGlobalsArgs,
        ) -> *mut std::os::raw::c_void,
    >,
    pub release_area:
        Option<unsafe fn(aper: &manageable_aperture_t, addr: *mut std::os::raw::c_void, size: u64)>,
    // void *(*allocate_area_aligned)(manageable_aperture_t *aper, void *addr, uint64_t size, uint64_t align);
    // void (*release_area)(manageable_aperture_t *aper, void *addr, uint64_t size);
}

#[derive(Debug)]
pub struct manageable_aperture<'a> {
    pub base: *mut std::os::raw::c_void,
    pub limit: *mut std::os::raw::c_void,
    pub align: u64,
    pub guard_pages: u32,
    pub vm_ranges: vm_area_t<'a>,
    pub tree: rbtree_t<'a>,
    pub user_tree: rbtree_t<'a>,
    pub is_cpu_accessible: bool,
    // ops: &'a manageable_aperture_ops_t,
    pub ops: manageable_aperture_ops_t,
}

unsafe impl Send for manageable_aperture<'_> {}

impl Default for manageable_aperture<'_> {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
            limit: std::ptr::null_mut(),
            align: 0,
            guard_pages: 0,
            vm_ranges: vm_area {
                start: std::ptr::null_mut(),
                end: std::ptr::null_mut(),
                next: None,
                prev: None,
            },
            tree: rbtree_s {
                root: None,
                sentinel: None,
            },
            user_tree: rbtree_s {
                root: None,
                sentinel: None,
            },
            is_cpu_accessible: false,
            ops: manageable_aperture_ops_t {
                allocate_area_aligned: None,
                release_area: None,
            },
        }
    }
}

/* Memory manager for an aperture */
pub type manageable_aperture_t<'a> = manageable_aperture<'a>;

pub enum svm_aperture_type {
    SVM_DEFAULT = 0,
    SVM_COHERENT,
    SVM_APERTURE_NUM,
}

/* The main structure for dGPU Shared Virtual Memory Management */
#[derive(Debug)]
pub struct svm_t<'a> {
    /* Two apertures can have different MTypes (for coherency) */
    pub apertures: [manageable_aperture_t<'a>; 2],

    /* Pointers to apertures, may point to the same aperture on
     * GFXv9 and later, where MType is not based on apertures
     */
    // pub dgpu_aperture: Option<&'a manageable_aperture_t<'a>>,
    // pub dgpu_alt_aperture: Option<&'amanageable_aperture_t<'a>>,
    pub dgpu_aperture: *mut manageable_aperture_t<'a>,
    pub dgpu_alt_aperture: *mut manageable_aperture_t<'a>,

    /* whether to use userptr for paged memory */
    pub userptr_for_paged_mem: bool,

    /* whether to check userptrs on registration */
    pub check_userptr: bool,

    /* whether to check reserve svm on registration */
    pub reserve_svm: bool,

    /* whether all memory is coherent (GPU cache disabled) */
    pub disable_cache: bool,

    /* specifies the alignment size as PAGE_SIZE * 2^alignment_order */
    pub alignment_order: u32,
}

impl Default for svm_t<'_> {
    fn default() -> Self {
        Self {
            apertures: [
                manageable_aperture_t::default(),
                manageable_aperture_t::default(),
            ],
            dgpu_aperture: std::ptr::null_mut(),
            dgpu_alt_aperture: std::ptr::null_mut(),
            userptr_for_paged_mem: false,
            check_userptr: false,
            reserve_svm: false,
            disable_cache: false,
            alignment_order: 0,
        }
    }
}
impl svm_t<'_> {
    pub fn dgpu_aperture_index(&self) -> usize {
        SVM_DEFAULT as usize
    }

    pub fn dgpu_alt_aperture_index(&self) -> usize {
        SVM_DEFAULT as usize
    }
}

#[derive(Debug, Clone)]
pub struct aperture_t {
    pub base: *mut std::os::raw::c_void,
    pub limit: *mut std::os::raw::c_void,
}

#[derive(Debug)]
pub struct gpu_mem_t<'a> {
    pub gpu_id: u32,
    pub device_id: u32,
    pub node_id: u32,
    pub local_mem_size: u64,
    pub EngineId: HSA_ENGINE_ID,
    pub lds_aperture: aperture_t,
    pub scratch_aperture: aperture_t,
    pub mmio_aperture: aperture_t,
    pub scratch_physical: manageable_aperture_t<'a>, /* For dGPU, scratch physical is allocated from
                                                      * dgpu_aperture. When requested by RT, each
                                                      * GPU will get a differnt range
                                                      */
    pub gpuvm_aperture: manageable_aperture_t<'a>, /* used for GPUVM on APU, outsidethe canonical address range */
    pub drm_render_fd: i32,
    pub usable_peer_id_num: u32,
    pub usable_peer_id_array: Vec<u32>,
    pub drm_render_minor: u32,
}

unsafe impl Send for gpu_mem_t<'_> {}

impl Default for gpu_mem_t<'_> {
    fn default() -> Self {
        Self {
            gpu_id: 0,
            device_id: 0,
            node_id: 0,
            local_mem_size: 0,
            EngineId: HSA_ENGINE_ID { Value: 0 },
            lds_aperture: aperture_t {
                base: std::ptr::null_mut(),
                limit: std::ptr::null_mut(),
            },
            scratch_aperture: aperture_t {
                base: std::ptr::null_mut(),
                limit: std::ptr::null_mut(),
            },
            mmio_aperture: aperture_t {
                base: std::ptr::null_mut(),
                limit: std::ptr::null_mut(),
            },
            scratch_physical: manageable_aperture::default(),
            gpuvm_aperture: manageable_aperture::default(),
            drm_render_fd: 0,
            usable_peer_id_num: 0,
            usable_peer_id_array: vec![],
            drm_render_minor: 0,
        }
    }
}

/* The VMs from DRM render nodes are used by KFD for the lifetime of
 * the process. Therefore we have to keep using the same FDs for the
 * lifetime of the process, even when we close and reopen KFD. There
 * are up to 128 render nodes that we cache in this array.
 */
pub const DRM_FIRST_RENDER_NODE: usize = 128;
pub const DRM_LAST_RENDER_NODE: usize = 255;

#[derive(Debug)]
pub struct HsaKmtFmmGlobal<'a> {
    pub drm_render_fds: [i32; DRM_LAST_RENDER_NODE + 1 - DRM_FIRST_RENDER_NODE],
    pub amdgpu_handle: [amdgpu_device; DRM_LAST_RENDER_NODE + 1 - DRM_FIRST_RENDER_NODE],
    pub svm: svm_t<'a>,
    /* The other apertures are specific to each GPU. gpu_mem_t manages GPU
     * specific memory apertures.
     */
    pub gpu_mem: Vec<gpu_mem_t<'a>>,
    pub gpu_mem_count: u32,
    pub g_first_gpu_mem: gpu_mem_t<'a>,
    /* GPU node array for default mappings */
    pub all_gpu_id_array_sizea: u32,
    pub all_gpu_id_array: Vec<u32>,
    pub dgpu_shared_aperture_base: *mut std::os::raw::c_void,
    pub dgpu_shared_aperture_limit: *mut std::os::raw::c_void,
}

unsafe impl Send for HsaKmtFmmGlobal<'_> {}

impl Clone for HsaKmtFmmGlobal<'_> {
    fn clone(&self) -> Self {
        Self {
            drm_render_fds: self.drm_render_fds,
            amdgpu_handle: self.amdgpu_handle,
            svm: svm_t::default(),
            gpu_mem: vec![],
            gpu_mem_count: self.gpu_mem_count,
            g_first_gpu_mem: gpu_mem_t::default(),
            all_gpu_id_array_sizea: 0,
            all_gpu_id_array: vec![],
            dgpu_shared_aperture_base: std::ptr::null_mut(),
            dgpu_shared_aperture_limit: std::ptr::null_mut(),
        }
    }
}

// #[derive(Debug)]
pub struct vm_object<'a> {
    pub start: *mut std::os::raw::c_void,
    pub userptr: *mut std::os::raw::c_void,
    pub userptr_size: u64,
    pub size: u64,   /* size allocated on GPU. When the user requests a random
                     	* size, Thunk aligns it to page size and allocates this
                     	* aligned size on GPU
                     	*/
    pub handle: u64, /* opaque */
    pub node_id: u32,
    pub node: rbtree_node_t<'a>,
    pub user_node: rbtree_node_t<'a>,

    pub mflags: HsaMemFlags, /* memory allocation flags */
    /* Registered nodes to map on SVM mGPU */
    pub registered_device_id_array: *mut u32,
    pub registered_device_id_array_size: u32,
    pub registered_node_id_array: *mut u32,
    pub registration_count: u32, /* the same memory region can be registered multiple times */
    /* Nodes that mapped already */
    pub mapped_device_id_array: *mut u32,
    pub mapped_device_id_array_size: u32,
    pub mapped_node_id_array: *mut u32,
    pub mapping_count: u32,
    /* Metadata of imported graphics buffers */
    pub metadata: *mut std::os::raw::c_void,
    /* User data associated with the memory */
    pub user_data: *mut std::os::raw::c_void,
    /* Flag to indicate imported KFD buffer */
    pub is_imported_kfd_bo: bool,
}

pub type vm_object_t<'a> = vm_object<'a>;

impl Default for vm_object<'_> {
    fn default() -> Self {
        Self {
            start: std::ptr::null_mut(),
            userptr: std::ptr::null_mut(),
            userptr_size: 0,
            size: 0,
            handle: 0,
            node_id: 0,
            node: Default::default(),
            user_node: Default::default(),
            mflags: HsaMemFlags {
                st: HsaMemFlagUnion { Value: 0 },
            },
            registered_device_id_array: std::ptr::null_mut(),
            registered_device_id_array_size: 0,
            registered_node_id_array: std::ptr::null_mut(),
            registration_count: 0,
            mapped_device_id_array: std::ptr::null_mut(),
            mapped_device_id_array_size: 0,
            mapped_node_id_array: std::ptr::null_mut(),
            mapping_count: 0,
            metadata: std::ptr::null_mut(),
            user_data: std::ptr::null_mut(),
            is_imported_kfd_bo: false,
        }
    }
}
