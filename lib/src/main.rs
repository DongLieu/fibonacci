use alloy_primitives::Bytes;
use fibonacci_lib::fibonacci;
use fibonacci_lib::PublicValuesStruct;

fn main() {
    // Ví dụ sử dụng hàm `fibonacci`
    let n: u32 = 123;
    let n_as_string: String = format!("{}", n);
    let n_as_bytes: Vec<u8> = n.to_be_bytes().to_vec();

    let (signature) = fibonacci(&n_as_string);

    // Chuyển đổi Vec<u8> thành Bytes
    let signature_bytes = Bytes::from(signature.clone());
    let n_as_bytes_bytes = Bytes::from(n_as_bytes.clone());

    let public_values = PublicValuesStruct {
        n: n_as_bytes_bytes,
        b: signature_bytes,
    };
    println!("pub value n: {:?}", public_values.n.to_vec());
    println!("pub value b: {:?}", public_values.b.to_vec());
}
