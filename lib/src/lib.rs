use alloy_sol_types::sol;
// use criterion::{criterion_group, Criterion};
use ed25519_dalek::{Signature, Signer, SigningKey};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng; // Import trait Rng để sử dụng phương thức `fill`

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        bytes n;
        bytes b; // Thay đổi từ `Signature` thành `bytes` vì `Signature` không phải là kiểu Solidity hợp lệ
    }
}

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
pub fn fibonacci(n: &str) -> (Vec<u8>) {
    let mut csprng: ThreadRng = thread_rng();
    let mut secret_key_bytes = [0u8; 32];
    csprng.fill(&mut secret_key_bytes); // random

    // Tạo SigningKey từ mảng bytes
    let signing_key: SigningKey = SigningKey::from_bytes(&secret_key_bytes);

    // convert
    let message: &[u8] = n.as_bytes();

    // sig
    let signature: Signature = signing_key.sign(message);

    // veryfi
    let verification_result = signing_key.verify(message, &signature);
    if verification_result.is_err() {
        panic!("Chữ ký không hợp lệ!");
    }
    (signature.to_vec())
}
