use vstd::prelude::*;

pub struct Foo;

#[verus_verify]
impl Foo {
    #[verus_spec(ret =>
        requires
            true,
        ensures
            ret == 1,
    )]
    fn test(a: u64, b: u64) -> u64 {
        1
    }
}

fn main() {
    println!("Test");
}