#![allow(non_snake_case)]

use crate::globals::HsakmtGlobals;
use crate::hsakmttypes::HsakmtStatus::{
    HSAKMT_STATUS_KERNEL_ALREADY_OPENED, HSAKMT_STATUS_KERNEL_IO_CHANNEL_NOT_OPENED,
    HSAKMT_STATUS_SUCCESS,
};
use crate::hsakmttypes::{HsaSystemProperties, HsakmtStatus};
use libc::{
    close, dlerror, dlsym, getenv, open, strcmp, sysconf, O_CLOEXEC, O_RDWR, RTLD_DEFAULT,
    _SC_PAGESIZE,
};
use std::ffi::CString;

pub const KFD_DEVICE_NAME: &str = "/dev/kfd";

pub fn ffs(n: i32) -> u32 {
    (n & -n).ilog2() + 1
}

impl HsakmtGlobals {
    pub unsafe fn init_page_size(&mut self) {
        let hsakmt_page_size = sysconf(_SC_PAGESIZE) as i32;

        self.hsakmt_page_size = hsakmt_page_size;
        self.hsakmt_page_shift = (ffs(hsakmt_page_size) - 1) as i32;
    }

    pub unsafe fn hsaKmtOpenKFD(&mut self) -> HsakmtStatus {
        let mut fd = -1;
        let mut sys_props = HsaSystemProperties::default();

        if self.hsakmt_kfd_open_count == 0 {
            let symbol_name = CString::new("amdgpu_device_get_fd").unwrap();

            let hsakmt_fn_amdgpu_device_get_fd = dlsym(RTLD_DEFAULT, symbol_name.as_ptr());
            let error = dlerror();

            if !error.is_null() {
                println!("amdgpu_device_get_fd is not available: {:?}", error);
            } else {
                println!(
                    "amdgpu_device_get_fd is available: {:?}",
                    hsakmt_fn_amdgpu_device_get_fd
                );
            }

            if self.hsakmt_kfd_fd < 0 {
                let kfd_device_name = CString::new(KFD_DEVICE_NAME).unwrap();
                fd = open(kfd_device_name.as_ptr(), O_RDWR | O_CLOEXEC);

                if fd == -1 {
                    close(fd);
                    return HSAKMT_STATUS_KERNEL_IO_CHANNEL_NOT_OPENED;
                }

                self.hsakmt_kfd_fd = fd;
            }

            self.init_page_size();

            let ret = self.hsakmt_init_kfd_version();
            if ret != HSAKMT_STATUS_SUCCESS {
                close(fd);
            }

            let ev = CString::new("HSA_USE_SVM").unwrap();

            let use_svm_str = getenv(ev.as_ptr());

            let ct = CString::new("0").unwrap();
            #[allow(clippy::nonminimal_bool)]
            let hsakmt_is_svm_api_supported =
                !(!use_svm_str.is_null() && strcmp(use_svm_str, ct.as_ptr()) == 0);
            self.hsakmt_is_svm_api_supported = hsakmt_is_svm_api_supported;

            let ret = self.hsakmt_topology_sysfs_get_system_props(&mut sys_props);
            if ret != HSAKMT_STATUS_SUCCESS {
                close(fd);
            }

            self.hsakmt_kfd_open_count += 1;

            // hsakmt_init_device_debugging_memory

            // hsakmt_init_counter_props
        } else {
            self.hsakmt_kfd_open_count += 1;
            return HSAKMT_STATUS_KERNEL_ALREADY_OPENED;
        }

        HSAKMT_STATUS_SUCCESS
    }

    pub unsafe fn hsaKmtCloseKFD(&self) -> HsakmtStatus {
        // ...
        HSAKMT_STATUS_SUCCESS
    }
}
