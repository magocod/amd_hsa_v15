#![allow(non_snake_case)]

use crate::globals::HsakmtGlobals;
use crate::hsakmttypes::HsakmtStatus::{
    HSAKMT_STATUS_DRIVER_MISMATCH, HSAKMT_STATUS_ERROR, HSAKMT_STATUS_SUCCESS,
};
use crate::hsakmttypes::{HsaVersionInfo, HsakmtStatus};
use crate::libhsakmt::hsakmt_ioctl;

#[derive(Debug, PartialEq)]
pub struct KfdIoctlGetVersionArgs {
    major_version: u32, /* from KFD */
    minor_version: u32, /* from KFD */
}

impl HsakmtGlobals {
    pub fn hsaKmtGetVersion(&self) -> HsaVersionInfo {
        self.version.kfd
    }

    pub unsafe fn hsakmt_init_kfd_version(&mut self) -> HsakmtStatus {
        let mut args = KfdIoctlGetVersionArgs {
            major_version: 0,
            minor_version: 0,
        };

        let hsakmt_kfd_fd = self.hsakmt_kfd_fd;

        let p_1 = ('K' as i32) << 8;
        let p_2 = std::mem::size_of::<KfdIoctlGetVersionArgs>() << (8 + 8);
        let amdkfd_ioc_get_version = ((2) << ((8 + 8) + 14)) | p_1 | ((0x01) << 0) | p_2 as i32;

        // macro AMDKFD_IOC_GET_VERSION ???
        // (((2U) << (((0+8)+8)+14)) | ((('K')) << (0+8)) | (((0x01)) << 0) | ((((sizeof(struct kfd_ioctl_get_version_args)))) << ((0+8)+8)))
        if hsakmt_ioctl(
            hsakmt_kfd_fd,
            amdkfd_ioc_get_version as u64,
            &mut args as *mut _ as *mut std::os::raw::c_void,
        ) == -1
        {
            return HSAKMT_STATUS_ERROR;
        }

        self.version.kfd.KernelInterfaceMajorVersion = args.major_version;
        self.version.kfd.KernelInterfaceMinorVersion = args.minor_version;

        if args.major_version != 1 {
            return HSAKMT_STATUS_DRIVER_MISMATCH;
        }

        HSAKMT_STATUS_SUCCESS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hsakmttypes::HsakmtStatus::HSAKMT_STATUS_SUCCESS;

    #[test]
    fn test_hsakmt_get_version() {
        let mut hsakmt = HsakmtGlobals::new();

        unsafe {
            let ret = hsakmt.hsaKmtOpenKFD();
            assert_eq!(ret, HSAKMT_STATUS_SUCCESS);

            let version_info = hsakmt.hsaKmtGetVersion();
            println!("{:#?}", version_info);
            assert_ne!(
                version_info,
                HsaVersionInfo {
                    KernelInterfaceMajorVersion: 0,
                    KernelInterfaceMinorVersion: 0
                }
            );

            assert!(version_info.KernelInterfaceMajorVersion > 0);
        }
    }

    #[test]
    fn test_hsakmt_get_version_not_initialized() {
        let hsakmt = HsakmtGlobals::new();

        let version_info = hsakmt.hsaKmtGetVersion();
        println!("{:#?}", version_info);

        assert_eq!(
            version_info,
            HsaVersionInfo {
                KernelInterfaceMajorVersion: 0,
                KernelInterfaceMinorVersion: 0,
            }
        );
    }
}
