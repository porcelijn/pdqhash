
use crate::{DctOutput, DCT_OUTPUT_W_H};

pub enum Orientation {
    X,
    Y,
    Plus1,
    Minus1,
}

pub enum Transform {
    PassThrough,
    Rotate(i32),
    Flip(Orientation),
}

impl Transform {
    pub fn apply(&self, matrix: &mut DctOutput) {
        use Orientation::*;
        match self {
            Self::PassThrough | Self::Rotate(0) => {},
            Self::Rotate(90) => *matrix = rotate90(matrix),
            Self::Rotate(180) => rotate180(matrix),
            Self::Rotate(270) => *matrix = rotate270(matrix),
            Self::Rotate(angle) => {
                if 0 <= *angle && *angle < 360 {
                    unimplemented!("invalid angle");
                }
                if *angle < 0 {
                    Self::Rotate(360 + (*angle % 360)).apply(matrix);
                } else {
                    Self::Rotate(*angle % 360).apply(matrix);
                }
            },
            Self::Flip(X) => flip_x(matrix),
            Self::Flip(Y) => flip_y(matrix),
            Self::Flip(Plus1) => *matrix = flip_plus1(matrix),
            Self::Flip(Minus1) => *matrix = flip_minus1(matrix),
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

fn flip_x(matrix: &mut DctOutput) {
    for i in 0..DCT_OUTPUT_W_H {
        for j in 0..DCT_OUTPUT_W_H {
            if i & 1 == 0 {
                let cell = &mut matrix[i * DCT_OUTPUT_W_H + j];
                *cell = -*cell;
            }
        }
    }
}

fn flip_y(matrix: &mut DctOutput) {
    for i in 0..DCT_OUTPUT_W_H {
        for j in 0..DCT_OUTPUT_W_H {
            if j & 1 == 0 {
                let cell = &mut matrix[i * DCT_OUTPUT_W_H + j];
                *cell = -*cell;
            }
        }
    }
}

fn flip_plus1(input: &DctOutput) -> DctOutput {
    let mut result: DctOutput = [0.0; DCT_OUTPUT_W_H * DCT_OUTPUT_W_H];
    for i in 0..DCT_OUTPUT_W_H {
        for j in 0..DCT_OUTPUT_W_H {
            result[j * DCT_OUTPUT_W_H + i] = input[i * DCT_OUTPUT_W_H + j];
        }
    }
    result
}

fn flip_minus1(input: &DctOutput) -> DctOutput {
    let mut result: DctOutput = [0.0; DCT_OUTPUT_W_H * DCT_OUTPUT_W_H];
    for i in 0..DCT_OUTPUT_W_H {
        for j in 0..DCT_OUTPUT_W_H {
            if (i + j) & 1 != 0 {
                result[j * DCT_OUTPUT_W_H + i] = -input[i * DCT_OUTPUT_W_H + j];
            } else {
                result[j * DCT_OUTPUT_W_H + i] = input[i * DCT_OUTPUT_W_H + j];
            }
        }
    }
    result
}


