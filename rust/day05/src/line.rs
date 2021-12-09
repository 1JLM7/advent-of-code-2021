use num_traits::{Float, Num};

use crate::vec::{orientation, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line<T> {
    a: Vec2<T>,
    b: Vec2<T>,
}

impl<T> Line<T> {
    pub fn new(a: Vec2<T>, b: Vec2<T>) -> Self {
        Self { a, b }
    }

    pub(crate) fn points(&self) -> [&Vec2<T>; 2] {
        [&self.a, &self.b]
    }
}

impl<T: Copy + Num + Ord> Line<T> {
    pub fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    pub fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    pub fn on(&self, p: &Vec2<T>) -> bool {
        p.x <= T::max(self.a.x, self.b.x)
            && p.x >= T::min(self.a.x, self.b.x)
            && p.y <= T::max(self.a.y, self.b.y)
            && p.y >= T::min(self.a.y, self.b.y)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let o1 = orientation(&self.a, &self.b, &other.a);
        let o2 = orientation(&self.a, &self.b, &other.b);
        let o3 = orientation(&other.a, &other.b, &self.a);
        let o4 = orientation(&other.a, &other.b, &self.b);

        if let (Some(a), Some(b), Some(c), Some(d)) = (o1, o2, o3, o4) {
            if a != b && c != d {
                return true;
            }
        }
        if let None = o1 {
            if self.on(&other.a) {
                return true;
            }
        }
        if let None = o2 {
            if self.on(&other.a) {
                return true;
            }
        }
        if let None = o3 {
            if Line::new(other.a, self.a).on(&other.b) {
                return true;
            }
        }
        if let None = o4 {
            if Line::new(other.a, self.b).on(&other.b) {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use super::Vec2;

    #[test]
    fn test_no_isect() {
        let l1 = Line::new(Vec2::new(1, 1), Vec2::new(10, 1));
        let l2 = Line::new(Vec2::new(1, 2), Vec2::new(10, 2));

        assert!(!l1.intersects(&l2));
    }

    #[test]
    fn test_isect() {
        let l1 = Line::new(Vec2::new(10, 0), Vec2::new(0, 10));
        let l2 = Line::new(Vec2::new(0, 0), Vec2::new(10, 10));

        assert!(l1.intersects(&l2));
    }
}
