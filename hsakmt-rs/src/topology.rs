use crate::fmm_types::{DRM_FIRST_RENDER_NODE, DRM_LAST_RENDER_NODE};
use crate::globals::HsakmtGlobals;
use crate::hsakmttypes::HsakmtStatus::HSAKMT_STATUS_SUCCESS;
use crate::hsakmttypes::{HsaSystemProperties, HsakmtStatus};
use amdgpu_drm_sys::bindings::{amdgpu_device, amdgpu_device_initialize};
use libc::{open, EACCES, EINVAL, ENOENT, EPERM, O_CLOEXEC, O_RDWR};
use std::ffi::CString;
use std::mem::MaybeUninit;

impl HsakmtGlobals {
    pub unsafe fn hsakmt_open_drm_render_device(&mut self, minor: i32) -> i32 {
        if minor < DRM_FIRST_RENDER_NODE as i32 || minor > DRM_LAST_RENDER_NODE as i32 {
            println!(
                "DRM render minor {} out of range [{}, {}]\n",
                minor, DRM_FIRST_RENDER_NODE, DRM_LAST_RENDER_NODE
            );
            return -EINVAL;
        }

        let index = (minor - DRM_FIRST_RENDER_NODE as i32) as usize;

        /* If the render node was already opened, keep using the same FD */
        if self.fmm.drm_render_fds[index] != 0 {
            return self.fmm.drm_render_fds[index];
        }

        let path = format!("/dev/dri/renderD{}", minor);
        let path_cs = CString::new(path.as_str()).unwrap();

        // let fd = File::open(&path).unwrap();
        // println!("File fd {:?}", fd);

        let fd = open(path_cs.as_ptr(), O_RDWR | O_CLOEXEC);

        if fd < 0 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap();

            if errno != ENOENT && errno != EPERM {
                println!("Failed to open {:?} {:?}", path, errno);
                if errno == EACCES {
                    println!("Check user is in \"video\" group")
                }
            }
            return -errno;
        }

        self.fmm.drm_render_fds[index] = fd;

        let mut device_handle: MaybeUninit<amdgpu_device> = MaybeUninit::uninit();
        let mut major_drm: MaybeUninit<u32> = MaybeUninit::zeroed();
        let mut minor_drm: MaybeUninit<u32> = MaybeUninit::zeroed();

        let ret = amdgpu_device_initialize(
            fd,
            major_drm.as_mut_ptr(),
            minor_drm.as_mut_ptr(),
            &mut device_handle.as_mut_ptr(),
        );
        if ret != 0 {
            panic!("amdgpu_device_initialize failed");
        }

        fd
    }

    pub fn topology_sysfs_check_node_supported(&mut self, sysfs_node_id: usize) -> bool {
        let node = self
            .topology
            .sys_devices_virtual_kfd
            .nodes
            .iter()
            .find(|x| x.node_id == sysfs_node_id)
            .unwrap();

        /* Retrieve the GPU ID */
        if node.gpu_id == 0 {
            return true;
        }

        /* Retrieve the node properties */

        /* Open DRM Render device */
        let ret_value = unsafe {
            self.hsakmt_open_drm_render_device(node.properties.drm_render_minor.unwrap() as i32)
        };

        if ret_value > 0 {
            return true;
        } else if ret_value != -ENOENT && ret_value != -EPERM {
            // ret = HSAKMT_STATUS_ERROR;
        }

        false
    }

    pub unsafe fn hsakmt_topology_sysfs_get_system_props(
        &mut self,
        props: &mut HsaSystemProperties,
    ) -> HsakmtStatus {
        let kfd = &self.topology.sys_devices_virtual_kfd;

        props.PlatformOem = kfd.platform_oem as u32;
        props.PlatformId = kfd.platform_id as u32;
        props.PlatformRev = kfd.platform_rev as u32;

        /*
         * Discover the number of sysfs nodes:
         * Assuming that inside nodes folder there are only folders
         * which represent the node numbers
         */
        let num_sysfs_nodes = kfd.get_nodes().len();

        let mut ids = vec![];

        for i in 0..num_sysfs_nodes {
            let is_node_supported = self.topology_sysfs_check_node_supported(i);
            if is_node_supported {
                ids.push(i);
            }
        }

        props.NumNodes = ids.len() as u32;

        self.topology.map_user_to_sysfs_node_id_size = ids.len();
        self.topology.map_user_to_sysfs_node_id = ids;
        self.topology.num_sysfs_nodes = num_sysfs_nodes;

        HSAKMT_STATUS_SUCCESS
    }
}
