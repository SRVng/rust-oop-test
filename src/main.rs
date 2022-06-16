use num::FromPrimitive;
use std::ops::{Add, Div, Mul, Sub};

trait Shape<T> {
    fn perimeter(&self) -> T;
    fn area(&self) -> T;
}

struct Rectangle<T> {
    width: T,
    length: T,
}

impl<
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + FromPrimitive
            + Copy,
    > Shape<T> for Rectangle<T>
{
    fn perimeter(&self) -> T {
        T::from_f32(2.0).expect("Failed to parse") * (self.width + self.length)
    }
    fn area(&self) -> T {
        self.width * self.length
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_perimeter() {
        let rect = Rectangle {
            width: 2,
            length: 3,
        };

        assert_eq!(rect.perimeter(), 10)
    }

    #[test]
    fn rectangle_area() {
        let rect = Rectangle {
            width: 2,
            length: 3,
        };

        assert_eq!(rect.area(), 6)
    }
}
