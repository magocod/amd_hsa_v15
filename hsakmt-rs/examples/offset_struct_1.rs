#[repr(C)]
struct C {
    b: u8,
    f: u16,
}

fn main() {
    let c = C { b: 0, f: 0 };

    // cast to u8 pointers so we get offset in bytes
    let c_u8_ptr = &c as *const C as *const u8;
    let f_u8_ptr = &c.f as *const u16 as *const u8;

    let v = unsafe { f_u8_ptr.offset_from(c_u8_ptr) as usize };

    println!("{:x?}", v);
}
