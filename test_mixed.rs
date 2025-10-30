use vstd::prelude::*;

// Test standalone function
#[verus_spec(result => 
     requires 
         x < 50, 
     ensures 
         result < 100, 
)]
fn standalone_func(x: u32) -> u32 {
    x * 2
}

pub struct Calculator;

#[verus_verify]
impl Calculator {
    // Test impl method
    #[verus_spec(sum => 
         requires 
             a < 100, 
             b < 100, 
         ensures 
             sum < 200, 
    )]
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    // Test impl method with with clause
    #[verus_spec(product =>
        with
            Ghost(multiplier): Ghost<nat>
        requires
            x < 10,
            multiplier < 10,
        ensures
            product < 100,
    )]
    fn multiply(x: u32) -> u32 {
        x * 5
    }
}

#[verus_verify]
fn main() {
    let result1 = standalone_func(20);
    let result2 = Calculator::add(30, 40);
}