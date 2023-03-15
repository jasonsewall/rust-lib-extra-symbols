use lib1::some_struct;

#[no_mangle]
pub extern "C" fn bar(v: u8) -> u8 {
    v+2
}
