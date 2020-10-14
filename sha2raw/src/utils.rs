use std::convert::TryInto;

#[allow(unused_must_use)]
pub fn bits256_expand_to_bits512(bits256: &[u8]) -> [u8; 64] {
    assert_eq!(bits256.len(), 32, "invalid sha256 value.");
    let mut buffer = [0u8; 64];
    buffer[..32].copy_from_slice(bits256);

    let mut u32_vec: Vec<u32> = bits256
        .chunks(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().expect("slice with incorrect length")))
        .collect::<Vec<_>>();
    u32_vec.reverse();

    let _ = u32_vec.into_iter().enumerate().map(|(i, x)| {
        let val = !x;
        buffer[32 + i * 4 .. 32 + (i + 1) * 4].copy_from_slice(&u32::to_le_bytes(val)[..]);
        val
    }).collect::<Vec<_>>();

    buffer
}

#[cfg(test)]
mod tests {
    use crate::utils::bits256_expand_to_bits512;

    #[test]
    fn test_bits256_expand_to_bits512() {
        let inb = [0u8; 32];
        let out = bits256_expand_to_bits512(&inb[..]);
        println!("{:?}", hex::encode(&out[..]));
    }
}