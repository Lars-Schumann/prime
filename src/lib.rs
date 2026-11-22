/// Number of primes `<= u32::MAX`.
pub const PRIME_COUNT: usize = 203_280_221;

#[repr(align(4))]
struct AlignedLikeU32([u8; PRIME_COUNT * 4]);

#[used]
static BYTES: AlignedLikeU32 =
    AlignedLikeU32(*include_bytes!(concat!(env!("OUT_DIR"), "/primes.bin")));

/// Every prime `<= u32::MAX` sorted in ascending order.
pub static PRIMES: &[u32] = {
    // SAFETY: `BYTES` is properly aligned via `AlignedLikeU32`, holds `PRIME_COUNT * 4` bytes, and lives for `'static`
    unsafe { core::slice::from_raw_parts(BYTES.0.as_ptr().cast::<u32>(), PRIME_COUNT) }
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        assert_eq!(PRIMES.len(), PRIME_COUNT);
        assert_eq!(&PRIMES[..8], &[2, 3, 5, 7, 11, 13, 17, 19]);
        assert_eq!(*PRIMES.last().unwrap(), 4_294_967_291);
    }

    #[test]
    fn sorted_ascending() {
        assert!(PRIMES.array_windows::<2>().all(|[p1, p2]| p1 < p2));
    }
}
