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
const fn to_u64(x: &[u8]) -> u64 {
    assert!(x.len() == 8);
    // little endian
    (x[0] as u64) << (0 << 3) |
    (x[1] as u64) << (1 << 3) |
    (x[2] as u64) << (2 << 3) |
    (x[3] as u64) << (3 << 3) |
    (x[4] as u64) << (4 << 3) |
    (x[5] as u64) << (5 << 3) |
    (x[6] as u64) << (6 << 3) |
    (x[7] as u64) << (7 << 3)
}

#[cfg(not(target_arch = "x86_64"))]
mod generic {
    use super::to_u64;
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
        popcount64d(to_u64(&x[8*0..8*1])) +
        popcount64d(to_u64(&x[8*1..8*2])) +
        popcount64d(to_u64(&x[8*2..8*3])) +
        popcount64d(to_u64(&x[8*3..8*4]))
    }

    pub fn distance(x: &[u8; HASH_LENGTH], y: &[u8; HASH_LENGTH]) -> u32 {
        popcount64d(to_u64(&x[8*0..8*1]) ^ to_u64(&y[8*0..8*1])) +
        popcount64d(to_u64(&x[8*1..8*2]) ^ to_u64(&y[8*1..8*2])) +
        popcount64d(to_u64(&x[8*2..8*3]) ^ to_u64(&y[8*2..8*3])) +
        popcount64d(to_u64(&x[8*3..8*4]) ^ to_u64(&y[8*3..8*4]))
    }

} // generic

#[cfg(target_arch = "x86_64")]
mod x86_64 {
    use super::to_u64;
    use crate::HASH_LENGTH;

    // Use (intel/amd) POPCNT intrinsic, 1 cycle throughput, 3 cycle latency

    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
    pub unsafe fn weight(x: &[u8; HASH_LENGTH]) -> u32 {
        to_u64(&x[8*0..8*1]).count_ones() +
        to_u64(&x[8*1..8*2]).count_ones() +
        to_u64(&x[8*2..8*3]).count_ones() +
        to_u64(&x[8*3..8*4]).count_ones()
    }

    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
    pub unsafe fn distance(x: &[u8; HASH_LENGTH], y: &[u8; HASH_LENGTH]) -> u32 {
        (to_u64(&x[8*0..8*1]) ^ to_u64(&y[8*0..8*1])).count_ones() +
        (to_u64(&x[8*1..8*2]) ^ to_u64(&y[8*1..8*2])).count_ones() +
        (to_u64(&x[8*2..8*3]) ^ to_u64(&y[8*2..8*3])).count_ones() +
        (to_u64(&x[8*3..8*4]) ^ to_u64(&y[8*3..8*4])).count_ones()
    }

} // x86_64

