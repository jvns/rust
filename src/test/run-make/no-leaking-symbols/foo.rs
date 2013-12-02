#[crate_type = "dylib"];

#[link(name = "c1", kind = "static")]
extern {
    fn cfunc() -> i32;
}

pub fn try() {
    unsafe {
        assert_eq!(cfunc(), 1);
    }
}
