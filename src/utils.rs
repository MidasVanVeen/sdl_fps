use std::cmp::Ord;

pub fn min<T: Ord>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn max<T: Ord>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

pub fn sign<T: Ord>(a: f32) -> i32 {
    if a < 0.0 {
        -1
    } else if a > 0.0 {
        1
    } else {
        0
    }
}
