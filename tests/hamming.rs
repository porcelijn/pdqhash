// Poor mans's Hamming distance for test fixtures
pub fn distance(a: &str, b: &str) -> usize {
    let a = hex::decode(a).unwrap();
    let b = hex::decode(b).unwrap();
    assert_eq!(a.len(), 32);
    assert_eq!(b.len(), 32);
    std::iter::zip(a.iter(), b.iter()).map(|(a,b)| a != b).filter(|v|*v).count()
}
