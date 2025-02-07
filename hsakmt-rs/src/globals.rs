#![allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    non_upper_case_globals,
    clippy::enum_clike_unportable_variant,
    clippy::mixed_case_hex_literals
)]

use crate::fmm::START_NON_CANONICAL_ADDR;
use crate::fmm_types::{
    gpu_mem_t, manageable_aperture_t, svm_t, DRM_FIRST_RENDER_NODE, DRM_LAST_RENDER_NODE,
};
use crate::hsakmttypes::{node_props_t, HsaSystemProperties, HsaVersionInfo};
use crate::topology_utils::SysDevicesVirtualKfd;
use amdgpu_drm_sys::bindings::amdgpu_device;

#[derive(Debug)]
pub struct TopologyGlobals {
    pub g_system: HsaSystemProperties,
    pub g_props: Vec<node_props_t>,
    /* This array caches sysfs based node IDs of CPU nodes + all supported GPU nodes.
     * It will be used to map user-node IDs to sysfs-node IDs.
     */
    pub map_user_to_sysfs_node_id: Vec<usize>,
    pub map_user_to_sysfs_node_id_size: usize,
    pub num_sysfs_nodes: usize,
    // utils
    pub sys_devices_virtual_kfd: SysDevicesVirtualKfd,
}

impl TopologyGlobals {
    pub fn new() -> Self {
        let mut sys_devices_virtual_kfd = SysDevicesVirtualKfd::new();
        sys_devices_virtual_kfd.load_nodes();

        Self {
            g_system: Default::default(),
            g_props: vec![],
            map_user_to_sysfs_node_id: vec![],
            map_user_to_sysfs_node_id_size: 0,
            num_sysfs_nodes: 0,
            sys_devices_virtual_kfd,
        }
    }
}

#[derive(Debug)]
pub struct FmmGlobals<'a> {
    pub drm_render_fds: [i32; DRM_LAST_RENDER_NODE + 1 - DRM_FIRST_RENDER_NODE],
    pub amdgpu_handle: [amdgpu_device; DRM_LAST_RENDER_NODE + 1 - DRM_FIRST_RENDER_NODE],
    pub svm: svm_t<'a>,
    /* The other apertures are specific to each GPU. gpu_mem_t manages GPU
     * specific memory apertures.
     */
    pub gpu_mem: Vec<gpu_mem_t<'a>>,
    pub gpu_mem_count: u32,
    // pub g_first_gpu_mem: gpu_mem_t<'a>,
    /* GPU node array for default mappings */
    pub all_gpu_id_array_size: u32,
    pub all_gpu_id_array: Vec<u32>,
    pub dgpu_shared_aperture_base: *mut std::os::raw::c_void,
    pub dgpu_shared_aperture_limit: *mut std::os::raw::c_void,
    /* On APU, for memory allocated on the system memory that GPU doesn't access
     * via GPU driver, they are not managed by GPUVM. cpuvm_aperture keeps track
     * of this part of memory.
     */
    pub cpuvm_aperture: manageable_aperture_t<'a>,
    /* mem_handle_aperture is used to generate memory handles
     * for allocations that don't have a valid virtual address
     * its size is 47bits.
     */
    pub mem_handle_aperture: manageable_aperture_t<'a>,
}

impl FmmGlobals<'_> {
    pub fn new() -> Self {
        Self {
            drm_render_fds: [0; DRM_LAST_RENDER_NODE + 1 - DRM_FIRST_RENDER_NODE],
            amdgpu_handle: [amdgpu_device { _unused: [] };
                DRM_LAST_RENDER_NODE + 1 - DRM_FIRST_RENDER_NODE],
            svm: svm_t::default(),
            gpu_mem: vec![],
            gpu_mem_count: 0,
            all_gpu_id_array_size: 0,
            all_gpu_id_array: vec![],
            dgpu_shared_aperture_base: std::ptr::null_mut(),
            dgpu_shared_aperture_limit: std::ptr::null_mut(),
            cpuvm_aperture: manageable_aperture_t::INIT_MANAGEABLE_APERTURE(0, 0),
            mem_handle_aperture: manageable_aperture_t::INIT_MANAGEABLE_APERTURE(
                START_NON_CANONICAL_ADDR as usize,
                (START_NON_CANONICAL_ADDR + (1 << 47)) as usize,
            ),
        }
    }
}

#[derive(Debug)]
pub struct VersionGlobals {
    pub kfd: HsaVersionInfo,
}

#[derive(Debug)]
pub struct HsakmtGlobals {
    pub fmm: FmmGlobals<'static>,
    pub topology: TopologyGlobals,
    pub version: VersionGlobals,
    // HSAKMT global data
    pub hsakmt_kfd_open_count: usize,
    pub hsakmt_kfd_fd: i32,
    pub hsakmt_system_properties_count: u64,
    // hsakmt_mutex
    pub hsakmt_is_dgpu: bool,
    pub hsakmt_page_size: i32,
    pub hsakmt_page_shift: i32,
    /* whether to check all dGPUs in the topology support SVM API */
    pub hsakmt_is_svm_api_supported: bool,
    /* zfb is mainly used during emulation */
    pub hsakmt_zfb_support: i32,
}

impl HsakmtGlobals {
    pub fn new() -> Self {
        Self {
            fmm: FmmGlobals::new(),
            topology: TopologyGlobals::new(),
            version: VersionGlobals {
                kfd: HsaVersionInfo {
                    KernelInterfaceMajorVersion: 0,
                    KernelInterfaceMinorVersion: 0,
                },
            },
            // HSAKMT global data
            hsakmt_kfd_fd: -1,
            hsakmt_kfd_open_count: 0,
            hsakmt_system_properties_count: 0,
            hsakmt_is_dgpu: false,
            hsakmt_page_size: 0,
            hsakmt_page_shift: 0,
            hsakmt_is_svm_api_supported: false,
            hsakmt_zfb_support: 0,
        }
    }

    pub fn check_kfd_open_and_panic(&self) {
        if self.hsakmt_kfd_open_count == 0 {
            panic!("HSAKMT_STATUS_KERNEL_IO_CHANNEL_NOT_OPENED");
        }
    }

    pub fn PAGE_SIZE(&self) -> i32 {
        self.hsakmt_page_size
    }
}
