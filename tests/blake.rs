use blake2_rfc::blake2b::Blake2b;
use blake2b_rs::Blake2bBuilder;
use hex_literal::hex;

#[test]
#[rustfmt::skip]
fn blake2b_with_key() {
    let key = hex!("
        000102030405060708090a0b0c0d0e0f
        101112131415161718191a1b1c1d1e1f
    ");

    let ctx = Blake2b::with_key(32, &key);
    let out0: [u8; 32] = ctx.finalize().as_bytes().try_into().expect("slice with incorrect length");
    println!("blake2-rfc: {:?}", out0);
    // [78, 81, 231, 169, 19, 252, 128, 19, 125, 165, 40, 128, 254, 204, 161, 117, 191, 129, 225, 23, 213, 198, 129, 38, 220, 39, 116, 3, 53, 23, 234, 13]

    let mut out1 = [0u8; 32];
    let hasher = Blake2bBuilder::new(32).key(&key).build();
    hasher.finalize(&mut out1);
    println!("blake2b-rs: {:?}", out1);
    // [78, 81, 231, 169, 19, 252, 128, 19, 125, 165, 40, 128, 254, 204, 161, 117, 191, 129, 225, 23, 213, 198, 129, 38, 220, 39, 116, 3, 53, 23, 234, 13]
}

#[test]
fn blake2b_without_key() {
    const PERSONALIZATION: &[u8] = b"sparsemerkletree";
    let mut ctx = Blake2b::new(32);
    ctx.update(PERSONALIZATION);
    let out0: [u8; 32] = ctx.finalize().as_bytes().try_into().expect("slice with incorrect length");
    println!("blake2-rfc: {:?}", out0);
    // [20, 40, 150, 3, 106, 99, 194, 240, 215, 8, 239, 35, 138, 184, 222, 228, 253, 91, 178, 51, 19, 207, 89, 164, 252, 54, 49, 3, 232, 160, 39, 158]

    let mut out1 = [0u8; 32];
    let mut hasher = Blake2bBuilder::new(32).build();
    hasher.update(PERSONALIZATION);
    hasher.finalize(&mut out1);
    println!("blake2b-rs: {:?}", out1);
    // [20, 40, 150, 3, 106, 99, 194, 240, 215, 8, 239, 35, 138, 184, 222, 228, 253, 91, 178, 51, 19, 207, 89, 164, 252, 54, 49, 3, 232, 160, 39, 158]
}
