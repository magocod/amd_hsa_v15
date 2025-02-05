use hsakmt_rs::test_kfd_utils::kfd_base_component::KFDBaseComponentTest;

#[test]
fn test_a() {
    let mut kfd_base = KFDBaseComponentTest::new();

    unsafe {
        kfd_base.set_up();
    }

    println!(
        "kfd_base.hsakmt.topology.g_props.len = {:?}",
        kfd_base.hsakmt.topology.g_props.len()
    );
    println!(
        "kfd_base.hsakmt.topology.g_props.len = {:?}",
        kfd_base.hsakmt.fmm.gpu_mem.len()
    );

    println!("assert test");
}
