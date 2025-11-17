use std::collections::BTreeMap;
use std::fmt::Display;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default, Copy, Eq)]
pub struct Point2D<T>(T, T);

pub trait Distance {
    fn distance(&self, rhs: &Self) -> f64;
}


pub trait Norm {
    fn norm(&self) -> f64;
    fn normalize(&self) -> Point2D<f64>;
}

impl <T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Point2D(x, y)
    }
}

impl <T: Copy> Point2D<T> {
    pub fn x(&self) -> T {
        self.0
    }

    pub fn y(&self) -> T {
        self.1
    }
}


impl <T: Display> Display for Point2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl <T: Neg<Output = T>> Neg for Point2D<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point2D(-self.0, -self.1)
    }
}

impl <T: Add<Output = T>> Add for Point2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl <T: Sub<Output = T>> Sub for Point2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl <T: Add<Output = T> + Copy> Add<T> for Point2D<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Point2D(self.0 + rhs, self.1 + rhs)
    }
}

impl <T: Sub<Output = T> + Copy> Sub<T> for Point2D<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Point2D(self.0 - rhs, self.1 - rhs)
    }
}

impl <T: Mul<Output = T> + Copy> Mul<T> for Point2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Point2D(self.0 * rhs, self.1 * rhs)
    }
}

impl <T: Div<Output = T> + Copy> Div<T> for Point2D<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Point2D(self.0 / rhs, self.1 / rhs)
    }
}

impl <T: Ord> Ord for Point2D<T> where Self: Norm {
    fn cmp(&self, other: &Self) -> Ordering {
        self.norm().partial_cmp(&other.norm()).unwrap()
    }
}

impl <T> IntoIterator for Point2D<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0, self.1].into_iter()
    }
}

impl <T: Into<f64> + Copy> Distance for Point2D<T> {
    fn distance(&self, rhs: &Self) -> f64 {
        let dx = rhs.0.into() - self.0.into();
        let dy = rhs.1.into() - self.1.into();
        (dx * dx + dy * dy).sqrt()
    }
}

impl <T: Into<f64> + Copy> Norm for Point2D<T> {
    fn norm(&self) -> f64 {
        let x: f64 = self.0.into();
        let y: f64 = self.1.into();
        (x * x + y * y).sqrt()
    }

    fn normalize(&self) -> Point2D<f64> {
       let norm = self.norm();
        Point2D(self.0.into() / norm, self.1.into() / norm)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Space<P, V> {
    points: BTreeMap<P, V>
}

impl <P, V> Space<P, V> {
    pub fn new() -> Self {
        Space { points: BTreeMap::new() }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}

impl <P: Ord, V> Space<P, V> {
    pub fn add(&mut self, point: P, value: V) {
        self.points.insert(point, value);
    }

    pub fn remove(&mut self, point: P) -> Option<V> {
        self.points.remove(&point)
    }
}

impl <P, V> IntoIterator for Space<P, V> {
    type Item = (P, V);
    type IntoIter = std::collections::btree_map::IntoIter<P, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

impl <P: Ord, V> Index<P> for Space<P, V> {
    type Output = V;

    fn index(&self, index: P) -> &Self::Output {
        &self.points[&index]
    }
}

impl <P: Ord, V> IndexMut<P> for Space<P, V> {
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        self.points.get_mut(&index).unwrap()
    }
}

pub fn main() {
    println!("Заглушка для main");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new_and_accessors() {
        let p = Point2D::new(3, 4);
        assert_eq!(p.x(), 3);
        assert_eq!(p.y(), 4);
    }

    #[test]
    fn test_display() {
        let p = Point2D::new(1, 2);
        assert_eq!(format!("{}", p), "(1; 2)");
    }

    #[test]
    fn test_neg() {
        let p = Point2D::new(2, -3);
        assert_eq!(-p, Point2D::new(-2, 3));
    }

    #[test]
    fn test_add_point() {
        let p1 = Point2D::new(1, 2);
        let p2 = Point2D::new(3, 4);
        assert_eq!(p1 + p2, Point2D::new(4, 6));
    }

    #[test]
    fn test_sub_point() {
        let p1 = Point2D::new(5, 7);
        let p2 = Point2D::new(2, 3);
        assert_eq!(p1 - p2, Point2D::new(3, 4));
    }

    #[test]
    fn test_add_scalar() {
        let p = Point2D::new(1, 2);
        assert_eq!(p + 3, Point2D::new(4, 5));
    }

    #[test]
    fn test_sub_scalar() {
        let p = Point2D::new(5, 7);
        assert_eq!(p - 2, Point2D::new(3, 5));
    }

    #[test]
    fn test_mul_scalar() {
        let p = Point2D::new(2, 3);
        assert_eq!(p * 4, Point2D::new(8, 12));
    }

    #[test]
    fn test_div_scalar() {
        let p = Point2D::new(8, 4);
        assert_eq!(p / 2, Point2D::new(4, 2));
    }

    #[test]
    fn test_into_iterator() {
        let p = Point2D::new(10, 20);
        let v: Vec<_> = p.into_iter().collect();
        assert_eq!(v, vec![10, 20]);
    }

    #[test]
    fn test_distance() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(3.0, 4.0);
        assert!((p1.distance(&p2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_norm_and_normalize() {
        let p = Point2D::new(3.0, 4.0);
        assert!((p.norm() - 5.0).abs() < 1e-10);
        let n = p.normalize();
        assert!((n.x() - 0.6).abs() < 1e-10);
        assert!((n.y() - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_ord_trait() {
        let p1 = Point2D::new(3.0, 4.0); // norm = 5
        let p2 = Point2D::new(6.0, 8.0); // norm = 10
        assert!(p1 < p2);
        assert!(p2 > p1);
        assert!(p1 == Point2D::new(3.0, 4.0));
    }

    #[test]
    fn test_space_new_and_len() {
        let s: Space<i32, &str> = Space::new();
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_space_add_and_len() {
        let mut s = Space::new();
        s.add(Point2D::<i32>::new(0, 0), "a");
        s.add(Point2D::<i32>::new(1, 1), "b");
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_space_remove() {
        let mut s = Space::new();
        s.add(Point2D::<i32>::new(0, 0), "a");
        assert_eq!(s.remove(Point2D::<i32>::new(0, 0)), Some("a"));
        assert_eq!(s.remove(Point2D::<i32>::new(0, 0)), None);
    }

    #[test]
    fn test_space_index_and_index_mut() {
        let mut s = Space::new();
        s.add(Point2D::<i32>::new(1, 1), 10);
        assert_eq!(s[Point2D::<i32>::new(1, 1)], 10);
        s[Point2D::<i32>::new(1, 1)] = 20;
        assert_eq!(s[Point2D::<i32>::new(1, 1)], 20);
    }

    #[test]
    fn test_space_into_iter() {
        let mut s = Space::new();
        s.add(Point2D::<i32>::new(1, 1), "a");
        s.add(Point2D::<i32>::new(2, 2), "b");
        let items: Vec<_> = s.into_iter().collect();
        assert_eq!(items, vec![
            (Point2D::<i32>::new(1, 1), "a"),
            (Point2D::<i32>::new(2, 2), "b")
        ]);
    }
}