use blake2_rfc::blake2b::Blake2b;

use crate::traits::Hasher;

pub struct Blake2bHasher(Blake2b);

impl Default for Blake2bHasher {
    fn default() -> Self {
        let blake2b = Blake2b::new(32);
        Self(blake2b)
    }
}

impl Hasher for Blake2bHasher {
    fn write_bytes(&mut self, b: &[u8]) {
        self.0.update(b);
    }

    fn write_byte(&mut self, b: u8) {
        self.0.update(&[b][..]);
    }

    fn write_h256(&mut self, h: &crate::h256::H256) {
        self.0.update(h.as_slice());
    }

    fn finish(self) -> crate::h256::H256 {
        let out: [u8; 32] = self.0.finalize().as_bytes().try_into().expect("finish failed");
        out.into()
    }
}