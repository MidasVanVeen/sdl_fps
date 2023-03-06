use num::Num;

pub fn min<T: Num + std::cmp::Ord>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn max<T: Num + std::cmp::Ord>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

pub fn sign<T: std::cmp::Ord>(a: f32) -> i32 {
    if a < 0.0 {
        -1
    } else if a > 0.0 {
        1
    } else {
        0
    }
}
