use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use auto_ops::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Self { x, y }
    }

    pub fn origin() -> Point {
        Self::new(0, 0)
    }

    pub fn up() -> Point {
        Self::new(0, -1)
    }

    pub fn down() -> Point {
        Self::new(0, 1)
    }

    pub fn left() -> Point {
        Self::new(-1, 0)
    }

    pub fn right() -> Point {
        Self::new(1, 0)
    }
}

impl From<&Point> for Point {
    fn from(p: &Point) -> Self { *p }
}

impl From<(i64, i64)> for Point {
    fn from(p: (i64, i64)) -> Point {
        Point { x: p.0, y: p.1 }
    }
}



impl_op_ex!(+ |a: &Point, b: &Point| -> Point { Point::new(a.x + b.x, a.y + b.y) });
impl_op!(+= |a: &mut Point, b: &Point| { *a = *a + b; });

impl_op_ex!(- |a: &Point, b: &Point| -> Point { Point::new(a.x - b.x, a.y - b.y) });
impl_op!(-= |a: &mut Point, b: &Point| { *a = *a - b; });

impl_op_ex!(* |a: &Point, b: i64| -> Point { Point::new(a.x * b, a.y * b) });
impl_op!(*= |a: &mut Point, b: i64| { *a = *a * b});

impl_op_ex!(- |a: &Point| -> Point { Point::new(-a.x, -a.y)});