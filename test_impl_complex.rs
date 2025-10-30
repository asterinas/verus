use vstd::prelude::*;

pub struct Foo;

#[verus_verify]
impl Foo {
    #[verus_spec(ret =>
        with
            Tracked(t1): Tracked<&int>,
            Ghost(g1): Ghost<nat>
        requires
            true,
        ensures
            ret == 1,
    )]
    fn complex_test(a: u64, b: u64) -> u64 {
        1
    }

    // Test without with clause
    #[verus_spec(ret =>
        requires
            a > 0,
        ensures
            ret > 0,
    )]
    fn simple_test(a: u64) -> u64 {
        a
    }
}

fn main() {}