use pdqhash::{generate_pdq_full_size, Transform::PassThrough};

#[test]
fn test_load() {
    fn load(data: &[u8]) -> String {
        let image = image::load_from_memory(data).unwrap();
        let hash = generate_pdq_full_size(&image, &PassThrough).0;
        hex::encode(hash)
    }

    assert_eq!(
        "f8f8f0cee0f4a84f06370a22038f63f0b36e2ed596621e1d33e6b39c4e9c9b22",
        load(include_bytes!("../test_data/bridge-1-original.jpg"))
    );
    assert_eq!(
        "30a10efd71cc3d429013d48d0ffffc52e34e0e17ada952a9d29685211ea9e5af",
        load(include_bytes!("../test_data/bridge-2-rotate-90.jpg"))
    );
    assert_eq!(
        "adad5a64b5a142e75b62a09857da895ae63b847fc23794b766b319361bc93188",
        load(include_bytes!("../test_data/bridge-3-rotate-180.jpg"))
    );
    assert_eq!(
        "a5f0a457a48995e8c9065c275aaa5498b61ba4bdf8fcf80387c32f8b1bfc4f05",
        load(include_bytes!("../test_data/bridge-4-rotate-270.jpg"))
    );
    assert_eq!(
        "f8f80f31e0f417b20e37f5cd028f980fb36ed02a9662c1e233e64c634e9c64dd",
        load(include_bytes!("../test_data/bridge-5-flipx.jpg"))
    );
    assert_eq!(
        "0dad2599b1a1bd1a5362576742da32a5e63b7380c2374b4866b366c91bc9ce77",
        load(include_bytes!("../test_data/bridge-6-flipy.jpg"))
    );
    assert_eq!(
        "f0a5e102f1ccc0bd945308720fff038de34ef1e8ada9a956d2967ade5ea91a50",
        load(include_bytes!("../test_data/bridge-7-flip-plus-1.jpg"))
    );
    assert_eq!(
        "a5f05aa8a4896a17c906a2d85aaaab07b61b5b42f8fc07fc87c3d0741bfcb0fa",
        load(include_bytes!("../test_data/bridge-8-flip-minus-1.jpg"))
    );
}


