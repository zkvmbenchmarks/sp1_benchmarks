//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

mod vec_sum10;
mod vec_sum100;
mod vec_sum10000;

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let test_name = sp1_zkvm::io::read::<String>();
    if test_name == "vecSum10" {
        let result: i32 = vec_sum10::test_func();
        sp1_zkvm::io::commit(&result);
    }
    else if test_name == "vecSum100" {
        let result: i32 = vec_sum100::test_func();
        sp1_zkvm::io::commit(&result);
    }
    else if test_name == "vecSum10000" {
        let result: i32 = vec_sum10000::test_func();
        sp1_zkvm::io::commit(&result);
    }
}
