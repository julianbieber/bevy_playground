use super::consts::{L1DIMENSION, L2DIMENSION, L3DIMENSION};

/// the overflow is calculated as the most significant bit of the next value (current - 1), if it underflows the msb is 1 otherwise 0
/// the next value for each dimension is capped to its defined range by ANDing the dimension size - 1
/// It only works under the assumption that the dimension sizes are powers of 2
/// Benchmarks showed that this version is faster then the branching implementation.
/// the optimized asm follows the same structure as the right/up version
pub fn left_or_down(x3: usize, x2: usize, x1: usize) -> Option<(usize, usize, usize)> {
    if x1 == 0 && x2 == 0 && x3 == 0 {
        return None;
    }

    let (right_x1, overflow_to_2) = {
        let next = x1 - 1;
        (
            next & (L1DIMENSION - 1),
            (next & (2 as usize).pow(usize::BITS - 1)) >> (usize::BITS - 1),
        )
    };

    let (right_x2, overflow_to_3) = {
        let next = x2 - overflow_to_2;
        (
            next & (L2DIMENSION - 1),
            (next & (2 as usize).pow(usize::BITS - 1)) >> (usize::BITS - 1),
        )
    };

    let right_x3 = x3 - overflow_to_3;
    Some((right_x3, right_x2, right_x1))
}

pub fn right_or_up(x3: usize, x2: usize, x1: usize) -> Option<(usize, usize, usize)> {
    if x1 == L1DIMENSION - 1 && x2 == L2DIMENSION - 1 && x3 == L3DIMENSION - 1 {
        return None;
    }

    let (right_x1, overflow_to_2) = {
        let next = x1 + 1;
        (next % L1DIMENSION, next / L1DIMENSION)
    };

    let (right_x2, overflow_to_3) = {
        let next = x2 + overflow_to_2;
        (next % L2DIMENSION, next / L2DIMENSION)
    };

    let right_x3 = x3 + overflow_to_3;
    Some((right_x3, right_x2, right_x1))
}
