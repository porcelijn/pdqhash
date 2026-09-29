// Quickly find the median
pub(crate) fn median(m: &[f32]) -> Option<f32> {
    let mut min = m.iter().cloned().reduce(f32::min)?;
    let mut max = m.iter().cloned().reduce(f32::max)?;

    let half = (m.len() + 1) / 2;
    loop {
        let guess = (min + max) / 2.0;
        let mut less = 0;
        let mut greater = 0;
        let mut equal = 0;
        let mut maxltguess = min;
        let mut mingtguess = max;
        for val in m {
            if *val < guess {
                less += 1;
                if *val > maxltguess {
                    maxltguess = *val;
                }
            } else if *val > guess {
                greater += 1;
                if *val < mingtguess {
                    mingtguess = *val;
                }
            } else {
                equal += 1;
            }
        }
        if less <= half && greater <= half {
            return Some(if less >= half {
                maxltguess
            } else if less + equal >= half {
                guess
            } else {
                mingtguess
            });
        } else if less > greater {
            max = maxltguess;
        } else {
            min = mingtguess;
        }
    }
}

