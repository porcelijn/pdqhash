
use crate::{DctOutput, DCT_OUTPUT_W_H};

pub enum Transform {
    PassThrough,
    Rotate90,
    Rotate180,
	// todo
}

impl Transform {
    pub fn apply(&self, matrix: &mut DctOutput) {
        match self {
            Self::PassThrough => {},
            Self::Rotate90 => *matrix = rotate90(matrix),
            Self::Rotate180 => rotate180(matrix),
            // todo
        }
    }
}

fn rotate90(input: &DctOutput) -> DctOutput {
    let mut result: DctOutput = [0.0; DCT_OUTPUT_W_H * DCT_OUTPUT_W_H];
    for i in 0..DCT_OUTPUT_W_H {
        for j in 0..DCT_OUTPUT_W_H {
            if j & 1 != 0 {
                result[j * DCT_OUTPUT_W_H + i] = input[i * DCT_OUTPUT_W_H + j];
            } else {
                result[j * DCT_OUTPUT_W_H + i] = -input[i * DCT_OUTPUT_W_H + j];
            }
        }
    }
    result
}

fn rotate180(matrix: &mut DctOutput) {
    for i in 0..DCT_OUTPUT_W_H {
        for j in 0..DCT_OUTPUT_W_H {
            if (i + j) & 1 != 0 {
                let cell = &mut matrix[i * DCT_OUTPUT_W_H + j];
                *cell = -*cell;
            }
        }
    }
}

