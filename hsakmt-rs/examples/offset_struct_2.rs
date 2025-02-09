use hsakmt_rs::fmm_types::vm_object_t;

#[repr(C)]
struct C {
    b: u8,
    f: u16,
}

pub fn main() {
    let offset_f: usize = unsafe {
        let c = std::mem::MaybeUninit::uninit();
        let c_ptr: *const C = c.as_ptr();

        // cast to u8 pointers so we get offset in bytes
        let c_u8_ptr = c_ptr as *const u8;
        let f_u8_ptr = std::ptr::addr_of!((*c_ptr).f) as *const u8;

        f_u8_ptr.offset_from(c_u8_ptr) as usize
    };

    println!("offset_f {}", offset_f);

    let offset_node: usize = unsafe {
        let c = std::mem::MaybeUninit::uninit();
        let c_ptr: *const vm_object_t = c.as_ptr();

        // cast to u8 pointers so we get offset in bytes
        let c_u8_ptr = c_ptr as *const u8;
        let f_u8_ptr = std::ptr::addr_of!((*c_ptr).node) as *const u8;

        f_u8_ptr.offset_from(c_u8_ptr) as usize
    };

    println!("offset_node {}", offset_node);

    let offset_user_node: usize = unsafe {
        let c = std::mem::MaybeUninit::uninit();
        let c_ptr: *const vm_object_t = c.as_ptr();

        // cast to u8 pointers so we get offset in bytes
        let c_u8_ptr = c_ptr as *const u8;
        let f_u8_ptr = std::ptr::addr_of!((*c_ptr).user_node) as *const u8;

        f_u8_ptr.offset_from(c_u8_ptr) as usize
    };

    println!("offset_user_node {}", offset_user_node);
}
