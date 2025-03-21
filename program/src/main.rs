//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_primitives::Bytes;
use alloy_sol_types::SolType;
use fibonacci_lib::{fibonacci, PublicValuesStruct};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let n = sp1_zkvm::io::read::<u32>();
    let n_as_string: String = format!("{}", n);
    let n_as_bytes: Vec<u8> = n.to_be_bytes().to_vec();

    // Compute the n'th fibonacci number using a function from the workspace lib crate.
    let (signature) = fibonacci(&n_as_string);

    let signature_bytes = Bytes::from(signature.clone());
    let n_as_bytes_bytes = Bytes::from(n_as_bytes.clone());

    // Encode the public values of the program.
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct {
        n: n_as_bytes_bytes,
        b: signature_bytes,
    });

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&bytes);
}
