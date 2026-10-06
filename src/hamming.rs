use crate::HASH_LENGTH;

pub fn weight(x: &[u8; HASH_LENGTH]) -> u32 {
    #[cfg(target_arch = "x86_64")]
    return unsafe { x86_64::weight(x) };
    #[cfg(not(target_arch = "x86_64"))]
    generic::weight(x)
}

pub fn distance(x: &[u8; HASH_LENGTH], y: &[u8; HASH_LENGTH]) -> u32 {
    #[cfg(target_arch = "x86_64")]
    return unsafe { x86_64::distance(x, y) };
    #[cfg(not(target_arch = "x86_64"))]
    generic::distance(x, y)
}

#[inline(always)]
fn get_u64<const N: usize>(x: &[u8; HASH_LENGTH]) -> u64 {
    use std::convert::TryInto;
    const MAX: usize = HASH_LENGTH / 8;
    assert!(N < MAX, "index out of bounds: N={} < {}", N, MAX);
    u64::from_le_bytes(x[8 * N .. 8 * (N + 1)].try_into().unwrap())
}

#[cfg(not(target_arch = "x86_64"))]
mod generic {
    use super::get_u64;
    use crate::HASH_LENGTH;

    // https://en.wikipedia.org/wiki/Hamming_weight
    // - popcount64d is better when most bits in x are 0
    // - this algorithm works the same for all data sizes
    // - this algorithm uses 3 arithmetic operations and 1 comparison/branch per "1" bit in x.
    const fn popcount64d(mut x: u64) -> u32 {
        let mut count = 0;
        while x != 0 {
            x &= x - 1;
            count += 1;
        }
        count
    }

    #[test]
    fn test_popcount64d() {
        assert_eq!(0, popcount64d(0));
        assert_eq!(64, popcount64d(!0));
        assert_eq!(2, popcount64d(10));
        assert_eq!(25, popcount64d(0b01001011101101011011110110100101001110000010110));
    }

    pub fn weight(x: &[u8; HASH_LENGTH]) -> u32 {
        popcount64d(get_u64::<0>(x)) +
        popcount64d(get_u64::<1>(x)) +
        popcount64d(get_u64::<2>(x)) +
        popcount64d(get_u64::<3>(x))
    }

    pub fn distance(x: &[u8; HASH_LENGTH], y: &[u8; HASH_LENGTH]) -> u32 {
        popcount64d(get_u64::<0>(x) ^ get_u64::<0>(y)) +
        popcount64d(get_u64::<1>(x) ^ get_u64::<1>(y)) +
        popcount64d(get_u64::<2>(x) ^ get_u64::<2>(y)) +
        popcount64d(get_u64::<3>(x) ^ get_u64::<3>(y))
    }

} // generic

#[cfg(target_arch = "x86_64")]
mod x86_64 {
    use super::get_u64;
    use crate::HASH_LENGTH;

    // Use (intel/amd) POPCNT intrinsic, 1 cycle throughput, 3 cycle latency

    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
    pub unsafe fn weight(x: &[u8; HASH_LENGTH]) -> u32 {
        get_u64::<0>(x).count_ones() +
        get_u64::<1>(x).count_ones() +
        get_u64::<2>(x).count_ones() +
        get_u64::<3>(x).count_ones()
    }

    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
    pub unsafe fn distance(x: &[u8; HASH_LENGTH], y: &[u8; HASH_LENGTH]) -> u32 {
        (get_u64::<0>(x) ^ get_u64::<0>(y)).count_ones() +
        (get_u64::<1>(x) ^ get_u64::<1>(y)).count_ones() +
        (get_u64::<2>(x) ^ get_u64::<2>(y)).count_ones() +
        (get_u64::<3>(x) ^ get_u64::<3>(y)).count_ones()
    }

    #[test]
    fn test_weight() {
        unsafe {
            assert_eq!(0,   weight(&[0; HASH_LENGTH]));
            assert_eq!(256, weight(&[!0; HASH_LENGTH]));
            assert_eq!(32,  weight(&[1; HASH_LENGTH]));
            assert_eq!(64,  weight(&[10; HASH_LENGTH]));
        }
    }

} // x86_64

