//! Compute PDQ hash of an image.
//! The PDQ algorithm was developed and open-sourced by Facebook (now Meta) in 2019.
//! It specifies a transformation which converts images into a binary format ('PDQ Hash') whereby
//! 'perceptually similar’ images produce similar outputs.
//! It was designed to offer an industry standard for representing images to collaborate on threat
//! mitigation.
use std::ops::Deref;

pub use image;

use image::GenericImageView;

const LUMA_FROM_R_COEFF: f32 = 0.299;
const LUMA_FROM_G_COEFF: f32 = 0.587;
const LUMA_FROM_B_COEFF: f32 = 0.114;

mod dct;
mod downscaling;
mod torben;
mod transform;

pub use transform::{Orientation, Transform};

//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Minimum size tested.
const MIN_HASHABLE_DIM: u32 = 5;

//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Tent filter.
const PDQ_NUM_JAROSZ_XY_PASSES: usize = 2;

const DOWNSAMPLE_DIMS: u32 = 512;

trait ToLuma: image::Pixel {
    fn to_luma(&self) -> f32;
}

impl ToLuma for image::Rgb<u8> {
    fn to_luma(&self) -> f32 {
        (self.0[0] as f32) * LUMA_FROM_R_COEFF
            + (self.0[1] as f32) * LUMA_FROM_G_COEFF
            + (self.0[2] as f32) * LUMA_FROM_B_COEFF
    }
}

impl ToLuma for image::Rgb<u16> {
    fn to_luma(&self) -> f32 {
        (self.0[0] as f32) / 256.0 * LUMA_FROM_R_COEFF
            + (self.0[1] as f32) / 256.0 * LUMA_FROM_G_COEFF
            + (self.0[2] as f32) / 256.0 * LUMA_FROM_B_COEFF
    }
}

impl ToLuma for image::Rgba<u8> {
    fn to_luma(&self) -> f32 {
        (self.0[0] as f32) * LUMA_FROM_R_COEFF
            + (self.0[1] as f32) * LUMA_FROM_G_COEFF
            + (self.0[2] as f32) * LUMA_FROM_B_COEFF
    }
}

impl ToLuma for image::Rgba<u16> {
    fn to_luma(&self) -> f32 {
        (self.0[0] as f32) / 256.0 * LUMA_FROM_R_COEFF
            + (self.0[1] as f32) / 256.0 * LUMA_FROM_G_COEFF
            + (self.0[2] as f32) / 256.0 * LUMA_FROM_B_COEFF
    }
}

impl ToLuma for image::Bgr<u8> {
    fn to_luma(&self) -> f32 {
        (self.0[0] as f32) * LUMA_FROM_B_COEFF
            + (self.0[1] as f32) * LUMA_FROM_G_COEFF
            + (self.0[2] as f32) * LUMA_FROM_R_COEFF
    }
}

impl ToLuma for image::Bgra<u8> {
    fn to_luma(&self) -> f32 {
        (self.0[0] as f32) * LUMA_FROM_B_COEFF
            + (self.0[1] as f32) * LUMA_FROM_G_COEFF
            + (self.0[2] as f32) * LUMA_FROM_R_COEFF
    }
}

impl ToLuma for image::Luma<u8> {
    fn to_luma(&self) -> f32 {
        self.0[0] as f32
    }
}

impl ToLuma for image::Luma<u16> {
    fn to_luma(&self) -> f32 {
        self.0[0] as f32 / 256.0
    }
}

impl ToLuma for image::LumaA<u8> {
    fn to_luma(&self) -> f32 {
        self.0[0] as f32
    }
}

impl ToLuma for image::LumaA<u16> {
    fn to_luma(&self) -> f32 {
        self.0[0] as f32 / 256.0
    }
}

trait ToLumaImage {
    fn to_luma_image(&self) -> (usize, usize, Vec<f32>);
}

impl<P, Container> ToLumaImage for image::ImageBuffer<P, Container>
where
    P: ToLuma + 'static,
    P::Subpixel: 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    fn to_luma_image(&self) -> (usize, usize, Vec<f32>) {
        let width = self.width();
        let height = self.height();
        let out = self.pixels().map(<P as ToLuma>::to_luma).collect();
        (width as usize, height as usize, out)
    }
}

fn to_luma_image(image: &image::DynamicImage) -> (usize, usize, Vec<f32>) {
    match image {
        image::DynamicImage::ImageLuma8(image) => image.to_luma_image(),
        image::DynamicImage::ImageLumaA8(image) => image.to_luma_image(),
        image::DynamicImage::ImageRgb8(image) => image.to_luma_image(),
        image::DynamicImage::ImageRgba8(image) => image.to_luma_image(),
        image::DynamicImage::ImageBgr8(image) => image.to_luma_image(),
        image::DynamicImage::ImageBgra8(image) => image.to_luma_image(),
        image::DynamicImage::ImageLuma16(image) => image.to_luma_image(),
        image::DynamicImage::ImageLumaA16(image) => image.to_luma_image(),
        image::DynamicImage::ImageRgb16(image) => image.to_luma_image(),
        image::DynamicImage::ImageRgba16(image) => image.to_luma_image(),
    }
}

// ----------------------------------------------------------------
fn decimate_float<const OUT_NUM_ROWS: usize, const OUT_NUM_COLS: usize>(
    input: &[f32], // matrix as in_num_rows x in_num_cols in row-major order
    in_num_rows: usize,
    in_num_cols: usize,
) -> [[f32; OUT_NUM_COLS]; OUT_NUM_ROWS] {
    let mut output = [[0.0; OUT_NUM_COLS]; OUT_NUM_ROWS];
    // target centers not corners:
    for outi in 0..OUT_NUM_ROWS {
        let ini = ((outi * 2 + 1) * in_num_rows) / (OUT_NUM_ROWS * 2);
        for outj in 0..OUT_NUM_COLS {
            let inj = ((outj * 2 + 1) * in_num_cols) / (OUT_NUM_COLS * 2);
            output[outi][outj] = input[ini * in_num_cols + inj];
        }
    }
    output
}

// ----------------------------------------------------------------
// This is all heuristic (see the PDQ hashing doc). Quantization matters since
// we want to count *significant* gradients, not just the sum of many small
// ones. The constants are all manually selected, and tuned as described in the
// document.
fn pdq_image_domain_quality_metric<const OUT_NUM_ROWS: usize, const OUT_NUM_COLS: usize>(
    buffer64x64: &[[f32; OUT_NUM_COLS]; OUT_NUM_ROWS],
) -> f32 {
    let mut gradient_sum = 0;

    for i in 0..(OUT_NUM_ROWS - 1) {
        for j in 0..OUT_NUM_COLS {
            let u = buffer64x64[i][j];
            let v = buffer64x64[i + 1][j];
            let d = (((u - v) * 100.0) / 255.0).trunc() as i32;
            gradient_sum += d.abs();
        }
    }
    for i in 0..OUT_NUM_ROWS {
        for j in 0..(OUT_NUM_COLS - 1) {
            let u = buffer64x64[i][j];
            let v = buffer64x64[i][j + 1];
            let d = (((u - v) * 100.0) / 255.0).trunc() as i32;
            gradient_sum += d.abs();
        }
    }

    // Heuristic scaling factor.
    let mut quality = gradient_sum / 90;
    if quality > 100 {
        quality = 100;
    }

    quality as f32
}

const BUFFER_W_H: usize = 64;

const DCT_OUTPUT_W_H: usize = 16;
type DctOutput = [f32; DCT_OUTPUT_W_H * DCT_OUTPUT_W_H];

const HASH_LENGTH: usize = DCT_OUTPUT_W_H * DCT_OUTPUT_W_H / 8;

fn pdq_buffer16x16_to_bits(input: &DctOutput) -> [u8; HASH_LENGTH] {
    let dct_median = torben::median(input).unwrap();
    let mut hash = [0; HASH_LENGTH];

    for i in 0..HASH_LENGTH {
        let mut byte = 0;
        for j in 0..8 {
            let val = input[i * 8 + j];
            if val > dct_median {
                byte |= 1 << j;
            }
        }
        hash[HASH_LENGTH - i - 1] = byte;
    }
    hash
}

/// Returns PDQ hash and quality of an image without first downscaling.
///
/// It is bit-for-bit compatible with the expected output from the Java version provided by facebook.
pub fn generate_pdq_full_size(image: &image::DynamicImage, transform: &Transform) -> ([u8; HASH_LENGTH], f32) {
    let (num_cols, num_rows, mut image) = to_luma_image(image);
    let window_size_along_rows = downscaling::compute_jarosz_filter_window_size(num_cols, BUFFER_W_H);
    let window_size_along_cols = downscaling::compute_jarosz_filter_window_size(num_rows, BUFFER_W_H);

    downscaling::jarosz_filter_float(
        image.as_mut_slice(),
        num_rows,
        num_cols,
        window_size_along_rows,
        window_size_along_cols,
        PDQ_NUM_JAROSZ_XY_PASSES,
    );

    let buffer64x64 =
        decimate_float::<BUFFER_W_H, BUFFER_W_H>(image.as_slice(), num_rows, num_cols);

    let mut buffer16x16 = dct::dct64_to_16(&buffer64x64);
    transform.apply(&mut buffer16x16);
    (
        pdq_buffer16x16_to_bits(&buffer16x16),
        pdq_image_domain_quality_metric(&buffer64x64),
    )
}

/// Returns PDQ hash and quality of an image.
///
/// Returns None if image is too small to generate a useful hash.
/// This will first downsize the image in RGB space using image crate, which is more efficient than
/// computing PDQ on the full size image. Some divergence from reference implementation is
/// expected.
pub fn generate_pdq(image: &image::DynamicImage, transform: &Transform) -> Option<([u8; HASH_LENGTH], f32)> {
    if image.width() < MIN_HASHABLE_DIM || image.height() < MIN_HASHABLE_DIM {
        return None;
    }

    let out = if image.width() > DOWNSAMPLE_DIMS || image.height() > DOWNSAMPLE_DIMS {
        generate_pdq_full_size(&image.thumbnail_exact(
            DOWNSAMPLE_DIMS.min(image.width()),
            DOWNSAMPLE_DIMS.min(image.height()),
        ),
        transform)
    } else {
        generate_pdq_full_size(image, transform)
    };
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Transform::*;

    fn hamming_distance(a: &str, b: &str) -> usize {
        let a = hex::decode(a).unwrap();
        let b = hex::decode(b).unwrap();
        assert_eq!(a.len(), HASH_LENGTH);
        assert_eq!(b.len(), HASH_LENGTH);
        std::iter::zip(a.iter(), b.iter()).map(|(a,b)| a != b).filter(|v|*v).count()
    }

    #[test]
    fn test_load() {
        fn load(data: &[u8]) -> String {
            let image = image::load_from_memory(data).unwrap();
            let hash = generate_pdq_full_size(&image, &PassThrough).0;
            hex::encode(hash)
        }

        assert_eq!(
            "f8f8f0cee0f4a84f06370a22038f63f0b36e2ed596621e1d33e6b39c4e9c9b22",
            load(include_bytes!("test_data/bridge-1-original.jpg"))
        );
        assert_eq!(
            "30a10efd71cc3d429013d48d0ffffc52e34e0e17ada952a9d29685211ea9e5af",
            load(include_bytes!("test_data/bridge-2-rotate-90.jpg"))
        );
        assert_eq!(
            "adad5a64b5a142e75b62a09857da895ae63b847fc23794b766b319361bc93188",
            load(include_bytes!("test_data/bridge-3-rotate-180.jpg"))
        );
        assert_eq!(
            "a5f0a457a48995e8c9065c275aaa5498b61ba4bdf8fcf80387c32f8b1bfc4f05",
            load(include_bytes!("test_data/bridge-4-rotate-270.jpg"))
        );
        assert_eq!(
            "f8f80f31e0f417b20e37f5cd028f980fb36ed02a9662c1e233e64c634e9c64dd",
            load(include_bytes!("test_data/bridge-5-flipx.jpg"))
        );
        assert_eq!(
            "0dad2599b1a1bd1a5362576742da32a5e63b7380c2374b4866b366c91bc9ce77",
            load(include_bytes!("test_data/bridge-6-flipy.jpg"))
        );
        assert_eq!(
            "f0a5e102f1ccc0bd945308720fff038de34ef1e8ada9a956d2967ade5ea91a50",
            load(include_bytes!("test_data/bridge-7-flip-plus-1.jpg"))
        );
        assert_eq!(
            "a5f05aa8a4896a17c906a2d85aaaab07b61b5b42f8fc07fc87c3d0741bfcb0fa",
            load(include_bytes!("test_data/bridge-8-flip-minus-1.jpg"))
        );
    }

    #[test]
    fn test_rotate() {
        fn rotate<const ANGLE: i32>(data: &[u8]) -> String {
            let image = image::load_from_memory(data).unwrap();
            let hash = generate_pdq_full_size(&image, &Rotate(ANGLE)).0;
            hex::encode(hash)
        }

        assert_eq!(
            hamming_distance(
                // hash copy-pasted from 'bridge-2-rotate-90.jpg' (in test_load)
                "30a10efd71cc3d429013d48d0ffffc52e34e0e17ada952a9d29685211ea9e5af",
                &rotate::<90>(include_bytes!("test_data/bridge-1-original.jpg"))),
            2
        );
 
        assert_eq!(
            hamming_distance(
                // hash copy-pasted from 'bridge-3-rotate-180.jpg' (in test_load)
                "adad5a64b5a142e75b62a09857da895ae63b847fc23794b766b319361bc93188",
                &rotate::<90>(include_bytes!("test_data/bridge-2-rotate-90.jpg"))),
            7
        );

        assert_eq!(
            hamming_distance(
                "f8f8f0cee0f4a84f06370a22038f63f0b36e2ed596621e1d33e6b39c4e9c9b22",
                &rotate::<90>(include_bytes!("test_data/bridge-4-rotate-270.jpg"))),
            8
        );

        assert_eq!(
            hamming_distance(
                // hash copy-pasted from 'bridge-3-rotate-180.jpg' (in test_load)
                "adad5a64b5a142e75b62a09857da895ae63b847fc23794b766b319361bc93188",
                &rotate::<180>(include_bytes!("test_data/bridge-1-original.jpg"))),
            7
        );

        assert_eq!(
            hamming_distance(
                // hash copy-pasted from 'bridge-4-rotate-270.jpg'
                "a5f0a457a48995e8c9065c275aaa5498b61ba4bdf8fcf80387c32f8b1bfc4f05",
                &rotate::<180>(include_bytes!("test_data/bridge-2-rotate-90.jpg"))),
            8
        );

        assert_eq!(
            hamming_distance(
                // hash copy-pasted from 'bridge-4-rotate-270.jpg'
                "a5f0a457a48995e8c9065c275aaa5498b61ba4bdf8fcf80387c32f8b1bfc4f05",
                &rotate::<270>(include_bytes!("test_data/bridge-1-original.jpg"))),
            8
        );
    }

    #[test]
    fn test_flip() {
        use transform::Orientation::*;

        fn flip(orientation: Orientation, data: &[u8]) -> String {
            let image = image::load_from_memory(data).unwrap();
            let hash = generate_pdq_full_size(&image, &Flip(orientation)).0;
            hex::encode(hash)
        }

        assert_eq!(
            hamming_distance(
                // hash copy-pasted from 'bridge-5-flipx.jpg'
                "f8f80f31e0f417b20e37f5cd028f980fb36ed02a9662c1e233e64c634e9c64dd",
                &flip(X, include_bytes!("test_data/bridge-1-original.jpg"))),
            7
        );

        assert_eq!(
            hamming_distance(
                // hash copy-pasted from 'bridge-6-flipy.jpg'
                "0dad2599b1a1bd1a5362576742da32a5e63b7380c2374b4866b366c91bc9ce77",
                &flip(Y, include_bytes!("test_data/bridge-1-original.jpg"))),
            2
        );

        // Perfect match with hash copy-pasted from 'bridge-7-flip-plus-1.jpg'
        assert_eq!(
            "f0a5e102f1ccc0bd945308720fff038de34ef1e8ada9a956d2967ade5ea91a50",
            &flip(Plus1, include_bytes!("test_data/bridge-1-original.jpg")),
        );
        
        assert_eq!(
            hamming_distance(
                // hash copy-pasted from 'bridge-8-flip-minus-1.jpg'
                "a5f05aa8a4896a17c906a2d85aaaab07b61b5b42f8fc07fc87c3d0741bfcb0fa",
                 &flip(Minus1, include_bytes!("test_data/bridge-1-original.jpg"))),
            7
        );
    }
}

