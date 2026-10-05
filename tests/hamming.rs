use pdqhash::hamming::distance as optimized_distance;

use std::convert::TryInto;
// Poor mans's Hamming distance for test fixtures
pub fn distance(a: &str, b: &str) -> u32 {
    fn decode(v: &str) -> [u8; 32] {
        let v = hex::decode(v).unwrap();
        assert_eq!(v.len(), 32);
        let v: [u8; 32] = v.try_into().unwrap();
        v
    }

    optimized_distance(&decode(a), &decode(b))
}
