
use crate::{DctOutput, DCT_OUTPUT_W_H};

pub enum Transform {
    PassThrough,
    Rotate(i32),
	// todo
}

impl Transform {
    pub fn apply(&self, matrix: &mut DctOutput) {
        match self {
            Self::PassThrough => {},
            Self::Rotate(90) => *matrix = rotate90(matrix),
            Self::Rotate(180) => rotate180(matrix),
            Self::Rotate(270) => *matrix = rotate270(matrix),
            Self::Rotate(_) => unimplemented!("invalid angle"),
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

fn rotate270(input: &DctOutput) -> DctOutput {
    let mut result: DctOutput = [0.0; DCT_OUTPUT_W_H * DCT_OUTPUT_W_H];
    for i in 0..DCT_OUTPUT_W_H {
        for j in 0..DCT_OUTPUT_W_H {
            if i & 1 != 0 {
                result[j * DCT_OUTPUT_W_H + i] = input[i * DCT_OUTPUT_W_H + j];
            } else {
                result[j * DCT_OUTPUT_W_H + i] = -input[i * DCT_OUTPUT_W_H + j];
            }
        }
    }
    result
}

