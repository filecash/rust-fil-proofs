use byteorder::{ByteOrder, BE};

use crate::consts::H512;
use crate::platform::Implementation;

lazy_static::lazy_static! {
    static ref IMPL: Implementation = Implementation::detect();
}

#[derive(Clone)]
pub struct Sha512 {
    len: (u64, u64),
    state: [u64; 8],
}

impl Default for Sha512 {
    fn default() -> Self {
        Sha512 {
            len: (0, 0),
            state: H512,
        }
    }
}

impl Sha512 {
    pub fn new() -> Self {
        Sha512::default()
    }

    pub fn digest(blocks: &[&[u8]]) -> [u8; 64] {
        let mut sha = Sha512::new();
        sha.input(blocks);
        sha.finish()
    }

    pub fn input(&mut self, blocks: &[&[u8]]) {
        debug_assert_eq!(blocks.len() % 2, 0, "invalid block length");

        let (res, over) = self.len.1.overflowing_add((blocks.len() as u64) << 9);
        self.len.1 = res;
        if over { self.len.0 += 1; }

        // self.len += (blocks.len() as u64) << 8;

        IMPL.compress512(&mut self.state, blocks);
    }

    pub fn finish(mut self) -> [u8; 64] {
        let mut block0 = [0u8; 64];
        let mut block1 = [0u8; 64];
        let (hi, lo) = self.len;

        // Append single 1 bit
        block0[0] = 0b1000_0000;

        // Write L as 64 big endian integer
        block1[64 - 16..64 - 8].copy_from_slice(&hi.to_be_bytes()[..]);
        block1[64 - 8..].copy_from_slice(&lo.to_be_bytes()[..]);

        IMPL.compress512(&mut self.state, &[&block0[..], &block1[..]][..]);

        let mut out = [0u8; 64];
        BE::write_u64_into(&self.state, &mut out);
        out
    }

    pub fn finish_with(mut self, block0: &[u8]) -> [u8; 64] {
        debug_assert_eq!(block0.len(), 64);

        let mut block1 = [0u8; 64];
        let (hi, lo) = self.len;

        // Append single 1 bit
        block1[0] = 0b1000_0000;

        // Write L as 64 big endian integer
        block1[64 - 16..64 - 8].copy_from_slice(&hi.to_be_bytes()[..]);
        block1[64 - 8..].copy_from_slice(&(lo + 512).to_be_bytes()[..]);

        IMPL.compress512(&mut self.state, &[block0, &block1[..]][..]);

        let mut out = [0u8; 64];
        BE::write_u64_into(&self.state, &mut out);
        out
    }
}

opaque_debug::implement!(Sha512);

#[cfg(test)]
mod tests {
    use super::*;

    use rand::{RngCore, SeedableRng};
    use rand_xorshift::XorShiftRng;
    use sha2::{Digest, Sha512 as Original};

    #[test]
    fn test_fuzz_simple_sha512() {
        fuzz(10);
    }

    #[test]
    #[ignore]
    fn test_fuzz_long_sha512() {
        fuzz(1_000);
    }

    fn fuzz(n: usize) {
        let rng = &mut XorShiftRng::from_seed([
            0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
            0xbc, 0xe5,
        ]);
        for k in 1..n {
            for _ in 0..100 {
                let mut input = vec![0u8; 128 * k];
                rng.fill_bytes(&mut input);
                let chunked = input.chunks(64).collect::<Vec<_>>();
                assert_eq!(&Sha512::digest(&chunked)[..], &Original::digest(&input)[..])
            }
        }

        for k in (1..n).step_by(2) {
            for _ in 0..100 {
                let mut input = vec![0u8; 64 * k];
                rng.fill_bytes(&mut input);
                let mut hasher = Sha512::new();
                for chunk in input.chunks(128) {
                    if chunk.len() == 128 {
                        hasher.input(&[&chunk[..64], &chunk[64..]]);
                    }
                }
                assert_eq!(input.len() % 128, 64);
                let hash = hasher.finish_with(&input[input.len() - 64..]);

                assert_eq!(
                    &hash[..],
                    &Original::digest(&input)[..],
                    "input: {:?}",
                    &input
                );
            }
        }
    }
} 
