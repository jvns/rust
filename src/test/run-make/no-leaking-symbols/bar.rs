extern mod foo;

#[link(name = "c2", kind = "static")]
extern {
    fn cfunc() -> i32;
}

fn main() {
    unsafe { assert_eq!(cfunc(), 2); }
    foo::try();
}

