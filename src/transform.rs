
use crate::{DctOutput, DCT_OUTPUT_W_H};

pub enum Transform {
    PassThrough,
    Rotate90,
	// todo
}

impl Transform {
    pub fn apply(&self, input: &DctOutput) -> DctOutput {
        match self {
            Self::PassThrough => *input,
            Self::Rotate90 => rotate90(input),
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

