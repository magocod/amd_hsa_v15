use crate::fmm_types::{DRM_FIRST_RENDER_NODE, DRM_LAST_RENDER_NODE};
use crate::hsakmttypes::{node_props_t, HsaSystemProperties, HsaVersionInfo};
use crate::topology_utils::SysDevicesVirtualKfd;
use amdgpu_drm_sys::bindings::amdgpu_device;

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

pub struct FmmGlobals {
    pub drm_render_fds: [i32; DRM_LAST_RENDER_NODE + 1 - DRM_FIRST_RENDER_NODE],
    pub amdgpu_handle: [amdgpu_device; DRM_LAST_RENDER_NODE + 1 - DRM_FIRST_RENDER_NODE],
}

impl FmmGlobals {
    pub fn new() -> Self {
        Self {
            drm_render_fds: [0; DRM_LAST_RENDER_NODE + 1 - DRM_FIRST_RENDER_NODE],
            amdgpu_handle: [amdgpu_device { _unused: [] };
                DRM_LAST_RENDER_NODE + 1 - DRM_FIRST_RENDER_NODE],
        }
    }
}

pub struct VersionGlobals {
    pub kfd: HsaVersionInfo,
}

pub struct HsakmtGlobals {
    pub fmm: FmmGlobals,
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
}
