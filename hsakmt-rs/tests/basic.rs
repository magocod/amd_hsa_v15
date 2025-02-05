use hsakmt_rs::globals::HsakmtGlobals;
use hsakmt_rs::hsakmttypes::HsaVersionInfo;
use hsakmt_rs::hsakmttypes::HsakmtStatus::HSAKMT_STATUS_SUCCESS;

#[test]
fn test_basic() {
    let mut hsakmt = HsakmtGlobals::new();

    unsafe {
        let ret = hsakmt.hsaKmtOpenKFD();
        assert_eq!(ret, HSAKMT_STATUS_SUCCESS);

        let version_info = hsakmt.hsaKmtGetVersion();
        println!("version_info: {:?}", version_info);
        assert_ne!(
            version_info,
            HsaVersionInfo {
                KernelInterfaceMajorVersion: 0,
                KernelInterfaceMinorVersion: 0
            }
        );

        let ret = hsakmt.hsaKmtCloseKFD();
        assert_eq!(ret, HSAKMT_STATUS_SUCCESS);
    }
}
