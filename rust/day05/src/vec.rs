use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use duplicate::duplicate;
use num_traits::{Float, Num};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[duplicate(
    trait_name  fn_name;
    [AddAssign] [add_assign];
    [SubAssign] [sub_assign];
    [MulAssign] [mul_assign];
    [DivAssign] [div_assign];
)]
impl<T: trait_name<T>> trait_name<Self> for Vec2<T> {
    fn fn_name(&mut self, rhs: Self) {
        self.x.fn_name(rhs.x);
        self.y.fn_name(rhs.y);
    }
}

#[duplicate(
    trait_name  fn_name;
    [MulAssign] [mul_assign];
    [DivAssign] [div_assign];
)]
impl<T: trait_name<T> + Copy> trait_name<T> for Vec2<T> {
    fn fn_name(&mut self, rhs: T) {
        self.x.fn_name(rhs);
        self.y.fn_name(rhs);
    }
}

#[duplicate(
    trait_name  trait_assign    fn_name fn_name_assign;
    [Add]       [AddAssign]     [add]   [add_assign];
    [Sub]       [SubAssign]     [sub]   [sub_assign];
    [Mul]       [MulAssign]     [mul]   [mul_assign];
    [Div]       [DivAssign]     [div]   [div_assign];
)]
impl<T: trait_assign<T>> trait_name<Self> for Vec2<T> {
    type Output = Self;
    fn fn_name(mut self, rhs: Self) -> Self::Output {
        self.fn_name_assign(rhs);
        self
    }
}

#[duplicate(
    trait_name  trait_assign    fn_name fn_name_assign;
    [Mul]       [MulAssign]     [mul]   [mul_assign];
    [Div]       [DivAssign]     [div]   [div_assign];
)]
impl<T: trait_assign<T> + Copy> trait_name<T> for Vec2<T> {
    type Output = Self;
    fn fn_name(mut self, rhs: T) -> Self::Output {
        self.fn_name_assign(rhs);
        self
    }
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy + Num> Vec2<T> {
    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }
}

pub fn orientation<T: Copy + Num + PartialOrd>(
    p: &Vec2<T>,
    q: &Vec2<T>,
    r: &Vec2<T>,
) -> Option<Orientation> {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if val.is_zero() {
        None
    } else {
        use Orientation::*;
        Some(if val > T::zero() { CW } else { CCW })
    }
}

pub fn boundary<T: Num + Ord>(
    points: impl Iterator<Item = Vec2<T>> + Clone,
) -> Option<(Vec2<T>, Vec2<T>)> {
    let min_x = points.clone().map(|v| v.x).reduce(T::min)?;
    let max_x = points.clone().map(|v| v.x).reduce(T::max)?;
    let min_y = points.clone().map(|v| v.y).reduce(T::min)?;
    let max_y = points.clone().map(|v| v.y).reduce(T::max)?;

    Some((Vec2::new(min_x, min_y), Vec2::new(max_x, max_y)))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Orientation {
    CW,
    CCW,
}
