use num::Float;
use num::Num;

#[derive(Copy, Clone, Debug)]
pub struct V2<T: Num + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy> V2<T> {
    pub fn new(a: T, b: T) -> V2<T> {
        V2 { x: a, y: b }
    }

    pub fn dot(&self, a: &V2<T>) -> T {
        self.x * a.x + self.y * a.y
    }
}

impl<T: Float> V2<T> {
    pub fn length(&self) -> T {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> V2<T> {
        let l = self.length();
        V2 {
            x: self.x / l,
            y: self.y / l,
        }
    }
}
