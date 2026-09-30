use pdqhash::{
    generate_pdq_full_size,
    Orientation,
    Orientation::*,
    Transform::Flip
};
mod hamming;

#[test]
fn test_flip() {

    fn flip(orientation: Orientation, data: &[u8]) -> String {
        let image = image::load_from_memory(data).unwrap();
        let hash = generate_pdq_full_size(&image, &Flip(orientation)).0;
        hex::encode(hash)
    }

    assert_eq!(
        hamming::distance(
            // hash copy-pasted from 'bridge-5-flipx.jpg'
            "f8f80f31e0f417b20e37f5cd028f980fb36ed02a9662c1e233e64c634e9c64dd",
            &flip(X, include_bytes!("../test_data/bridge-1-original.jpg"))),
        7);

    assert_eq!(
        hamming::distance(
            // hash copy-pasted from 'bridge-6-flipy.jpg'
            "0dad2599b1a1bd1a5362576742da32a5e63b7380c2374b4866b366c91bc9ce77",
            &flip(Y, include_bytes!("../test_data/bridge-1-original.jpg"))),
        2);

    // Perfect match with hash copy-pasted from 'bridge-7-flip-plus-1.jpg'
    assert_eq!(
        "f0a5e102f1ccc0bd945308720fff038de34ef1e8ada9a956d2967ade5ea91a50",
        &flip(Plus1, include_bytes!("../test_data/bridge-1-original.jpg")));

    assert_eq!(
        hamming::distance(
            // hash copy-pasted from 'bridge-8-flip-minus-1.jpg'
            "a5f05aa8a4896a17c906a2d85aaaab07b61b5b42f8fc07fc87c3d0741bfcb0fa",
            &flip(Minus1, include_bytes!("../test_data/bridge-1-original.jpg"))),
        7);
}

