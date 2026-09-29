pub fn compute_jarosz_filter_window_size(old_dimension: usize, new_dimension: usize) -> usize {
    (old_dimension + 2 * new_dimension - 1) / (2 * new_dimension)
}

pub fn jarosz_filter_float(
    buffer1: &mut [f32], // matrix as num_rows x num_cols in row-major order
    num_rows: usize,
    num_cols: usize,
    window_size_along_rows: usize,
    window_size_along_cols: usize,
    nreps: usize,
) {
    let mut temp_buf = Vec::new();
    temp_buf.resize(buffer1.len(), 0.0);
    for _ in 0..nreps {
        box_along_rows_float(
            buffer1,
            temp_buf.as_mut_slice(),
            num_rows,
            num_cols,
            window_size_along_rows,
        );
        box_along_cols_float(
            temp_buf.as_slice(),
            buffer1,
            num_rows,
            num_cols,
            window_size_along_cols,
        );
    }
}

// This is called from two places, one has a constant stride, the other a variable stride
// It should compile a version for each.
#[inline(always)]
fn box_one_d_float(
    invec: &[f32],
    in_start_offset: usize,
    outvec: &mut [f32],
    vector_length: usize,
    stride: usize,
    full_window_size: usize,
) {
    let half_window_size = (full_window_size + 2) / 2; // 7->4, 8->5

    let phase_1_nreps = half_window_size - 1;
    let phase_2_nreps = full_window_size - half_window_size + 1;

    let oi_off = phase_1_nreps * stride;
    let li_off = phase_2_nreps * stride;

    let mut sum = 0.0;
    let mut current_window_size = 0.0;

    let phase_1_end = oi_off + in_start_offset;

    // PHASE 1: ACCUMULATE FIRST SUM NO WRITES
    for ri in (in_start_offset..phase_1_end).step_by(stride) {
        let value = invec[ri];
        sum += value;
        current_window_size += 1.0;
    }

    let phase_2_end = full_window_size * stride + in_start_offset;
    // PHASE 2: INITIAL WRITES WITH SMALL WINDOW
    for ri in (phase_1_end..phase_2_end).step_by(stride) {
        let oi = ri - oi_off;
        sum += invec[ri];
        current_window_size += 1.0;
        outvec[oi] = sum / current_window_size;
    }

    let phase_3_end = vector_length * stride + in_start_offset;
    // PHASE 3: WRITES WITH FULL WINDOW
    for ri in (phase_2_end..phase_3_end).step_by(stride) {
        let oi = ri - oi_off;
        let li = oi - li_off;
        sum += invec[ri];
        sum -= invec[li];
        outvec[oi] = sum / (current_window_size);
    }

    let phase_4_start = (vector_length - half_window_size + 1) * stride + in_start_offset;
    // PHASE 4: FINAL WRITES WITH SMALL WINDOW
    for oi in (phase_4_start..phase_3_end).step_by(stride) {
        let li = oi - li_off;
        sum -= invec[li];
        current_window_size -= 1.0;
        outvec[oi] = sum / current_window_size;
    }
}

// ----------------------------------------------------------------
fn box_along_rows_float(
    input: &[f32],      // matrix as num_rows x num_cols in row-major order
    output: &mut [f32], // matrix as num_rows x num_cols in row-major order
    n_rows: usize,
    n_cols: usize,
    window_size: usize,
) {
    for i in 0..n_rows {
        box_one_d_float(input, i * n_cols, output, n_cols, 1, window_size);
    }
}

// ----------------------------------------------------------------
fn box_along_cols_float(
    input: &[f32],      // matrix as num_rows x num_cols in row-major order
    output: &mut [f32], // matrix as num_rows x num_cols in row-major order
    n_rows: usize,
    n_cols: usize,
    window_size: usize,
) {
    for j in 0..n_cols {
        box_one_d_float(input, j, output, n_rows, n_cols, window_size);
    }
}


