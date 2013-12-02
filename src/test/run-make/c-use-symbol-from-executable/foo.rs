use std::libc;

#[no_mangle]
pub extern "C" fn rust_foo() -> libc::c_int { 3 }

#[link(name = "cfoo", kind = "static")]
extern {
    fn c_foo() -> libc::c_int;
}

fn main() {
  unsafe { assert_eq!(c_foo(), 3) }
}
