use vstd::prelude::*;

// ANCHOR: verus_spec
#[verus_spec(sum => 
     requires 
         x < 100, 
         y < 100, 
     ensures 
         sum < 200, 
)]
fn my_exec_fun(x: u32, y: u32) -> u32 
{ 
    x + y 
}

#[verus_verify]
fn main() {
}