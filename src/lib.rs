#![no_std]

#[cfg(target_pointer_width = "16")]
compile_error!("32-bit target or greater needed for large pre-computed arrays.");

use core::mem::transmute;

/// Number of primes `<= u32::MAX`.
pub const PRIME_COUNT: usize = 203_280_221;

const SHIFT: u32 = 8;
const PI_LEN: usize = (1 << (32 - SHIFT)) + 1;

/// Every prime `<= u32::MAX` sorted in ascending order.
// SAFETY: `build.rs` writes exactly `PRIME_COUNT` `u32`s in target endianness (a size mismatch
// fails to compile), and every bit pattern is a valid `u32`.
pub static PRIMES: [u32; PRIME_COUNT] =
    unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/primes.bin"))) };

// `PI[k]` = number of primes `< k << SHIFT`.
// SAFETY: same as `PRIMES`, with `PI_LEN` `u32`s.
static PI: [u32; PI_LEN] =
    unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/pi.bin"))) };

pub fn prime_index(num: u32) -> Option<usize> {
    PRIMES.binary_search(&num).ok()
}

pub fn prime_index_smort(num: u32) -> Option<usize> {
    let k = (num >> SHIFT) as usize;
    PRIMES[PI[k] as usize..PI[k + 1] as usize]
        .binary_search(&num)
        .ok()
}

pub fn is_prime(num: u32) -> bool {
    prime_index(num).is_some()
}

pub fn is_prime_smort(num: u32) -> bool {
    prime_index_smort(num).is_some()
}

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

    #[test]
    fn pi_table() {
        for (k, &count) in PI.iter().enumerate() {
            let bound = (k as u64) << SHIFT;
            assert_eq!(
                count as usize,
                PRIMES.partition_point(|&p| (p as u64) < bound)
            );
        }
    }

    #[test]
    fn interpolation_matches_binary() {
        let mut state = 0x1234_5678_u64;
        let random = (0..1_000_000).map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 32) as u32
        });
        let edges = (0..=10_000).chain(u32::MAX - 10_000..=u32::MAX);
        for n in edges.chain(random).chain(PRIMES.iter().copied()) {
            assert_eq!(is_prime_smort(n), is_prime(n), "{n}");
        }
    }
}
