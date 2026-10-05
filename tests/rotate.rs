mod hamming;

use pdqhash::{generate_pdq_full_size, Transform::Rotate};

fn rotate<const ANGLE: i32>(data: &[u8]) -> String {
    let image = image::load_from_memory(data).unwrap();
    let hash = generate_pdq_full_size(&image, &Rotate(ANGLE)).0;
    hex::encode(hash)
}

#[test]
fn test_rotate90() {
    assert_eq!(
        hamming::distance(
            // hash copy-pasted from 'bridge-2-rotate-90.jpg' (in test_load)
            "30a10efd71cc3d429013d48d0ffffc52e34e0e17ada952a9d29685211ea9e5af",
            &rotate::<90>(include_bytes!("../test_data/bridge-1-original.jpg"))),
        2);

    assert_eq!(
        hamming::distance(
            // hash copy-pasted from 'bridge-3-rotate-180.jpg' (in test_load)
            "adad5a64b5a142e75b62a09857da895ae63b847fc23794b766b319361bc93188",
            &rotate::<90>(include_bytes!("../test_data/bridge-2-rotate-90.jpg"))),
        8);

    assert_eq!(
        hamming::distance(
            "f8f8f0cee0f4a84f06370a22038f63f0b36e2ed596621e1d33e6b39c4e9c9b22",
            &rotate::<90>(include_bytes!("../test_data/bridge-4-rotate-270.jpg"))),
        8);
}

#[test]
fn test_rotate180() {
    assert_eq!(
        hamming::distance(
            // hash copy-pasted from 'bridge-3-rotate-180.jpg' (in test_load)
            "adad5a64b5a142e75b62a09857da895ae63b847fc23794b766b319361bc93188",
            &rotate::<180>(include_bytes!("../test_data/bridge-1-original.jpg"))),
        8);

    assert_eq!(
        hamming::distance(
            // hash copy-pasted from 'bridge-4-rotate-270.jpg'
            "a5f0a457a48995e8c9065c275aaa5498b61ba4bdf8fcf80387c32f8b1bfc4f05",
            &rotate::<180>(include_bytes!("../test_data/bridge-2-rotate-90.jpg"))),
        8);
}

#[test]
fn test_rotate270() {
    assert_eq!(
        hamming::distance(
            // hash copy-pasted from 'bridge-4-rotate-270.jpg'
            "a5f0a457a48995e8c9065c275aaa5498b61ba4bdf8fcf80387c32f8b1bfc4f05",
            &rotate::<270>(include_bytes!("../test_data/bridge-1-original.jpg"))),
        8);

    assert_eq!(
        rotate::<-90>(include_bytes!("../test_data/bridge-6-flipy.jpg")),
        rotate::<270>(include_bytes!("../test_data/bridge-6-flipy.jpg")));
}


