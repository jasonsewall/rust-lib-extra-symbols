#[repr(C)]
pub struct some_struct {
    a: u8,
}

#[no_mangle]
pub extern "C" fn foo(v: u8) -> u8 {
    v+2
}
