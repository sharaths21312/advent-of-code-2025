use std::default;
use std::fmt::Display;
use std::iter::Enumerate;
use std::ops::Index;
use std::os::windows;

use crate::utils::point::Point;
use crate::utils::*;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>
}

impl<T, U: TryInto<usize>> Index<(U, U)> for Grid<T> {
    fn index(&self, index: (U, U)) -> &Self::Output {
        let x = match index.0.try_into() { Ok(x) => x, Err(_) => panic!("")};
        let y = match index.1.try_into() { Ok(x) => x, Err(_) => panic!("")};
        &self.data[self.width * y + x]
    }

    type Output = T;
}

impl<T> Index<&Point> for Grid<T> {
    fn index(&self, index: &Point) -> &Self::Output {
        &self.index((index.x, index.y))
   }

   type Output = T;
}

impl<'a, T> Grid<T> {
    pub fn make(source: Vec<T>, width: i64, height: i64) -> Grid<T> {
        assert_eq!(source.len(), (width * height).try_into().unwrap(), "Grid declaration uneven!");
        Grid { width: width as usize, height: height as usize, data: source }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn index_wrap(&self, index: &Point) -> &T {
        let w: i64 = self.width as i64;
        let h: i64 = self.height as i64;
        &self[(index.x.rem_euclid(w), index.y.rem_euclid(h))]
    }

    pub fn index_wrap_update(&self, index: &Point) -> (&T, Point) {
        let w: i64 = self.width as i64;
        let h: i64 = self.height as i64;
        let xnew = index.x.rem_euclid(w);
        let ynew = index.y.rem_euclid(h);
        (&self[(xnew, ynew)], Point::from((xnew, ynew)))
    }

    pub fn is_in_bounds(&self, point: &Point) -> bool {
        let x_bound = self.width as i64;
        let y_bound = self.height as i64;
        return point.x >= 0 && point.x < x_bound && point.y >= 0 && point.y < y_bound;
    }

    pub fn coords(&self, index: usize) -> Point {
        Point::new((index % self.width) as i64, (index / self.width) as i64)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn get(&self, pos: &Point) -> Option<&T> {
        if self.is_in_bounds(pos) {
            return Some(&self[pos]);
        } else {
            return None;
        }
    }

    pub fn gen_from_str(input: &str, f: fn(char) -> T) -> Grid<T> {
        let width = input.lines().next().unwrap().len();
        let height = input.len()/width;
        let data = input.chars().filter(|c| c.is_whitespace()).map(f).collect();
        Grid { width, height, data }
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Grid<T> {
        Grid { width: width, height: height, data: vec![default; width * height] }
    }

    pub fn get_or(&self, pos: &Point, default: T) -> T {
        self.get(pos).copied().unwrap_or(default)
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{}", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<&str> for Grid<char> {
    fn from(value: &str) -> Self {
        Self::gen_from_str(value, |c| c)
    }
}