use std::{fmt::{Display, Formatter, Result}, time::Duration};
use Solution::*;

pub enum Solution {
    I32(i32),
    I64(i64),
    Str(String)
}

pub struct FullSolution {
    pub part1: Solution,
    pub part2: Solution,
    pub time1: Duration,
    pub time2: Duration
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            I32(x) => x.fmt(f),
            I64(x) => x.fmt(f),
            Str(x) => x.fmt(f)
        }
    }
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    }
}

impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(String, Str);

impl From<&str> for Solution {
    fn from(sol: &str) -> Self {
        Self::Str(sol.to_owned())
    }
}