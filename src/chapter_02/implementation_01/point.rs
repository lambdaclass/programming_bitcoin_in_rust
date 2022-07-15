use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct Point<T> {
  x: Option<T>,
  y: Option<T>,
  a: i32,
  b: i32,
}

impl Point<i32> {
  pub fn new(x: Option<i32>, y: Option<i32>, a: i32, b: i32) -> Point<i32> {
    if let (Some(x_some), Some(y_some)) = (x, y) {
      if i32::pow(y_some, 2) != i32::pow(x_some, 3) + (a * x_some) + b {
        panic!("The point is not in the curve.")
      } else {
        return Point {x, y, a, b}
      }
    }
    Point{ x: None, y: None, a, b }
  }
}

impl PartialEq for Point<i32> {
  fn eq(self: &Self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
  }
}

impl Add for Point<i32> {
  type Output = Self;
  fn add(self: Self, other: Self) -> Self {
    if self.a != other.a || self.b != other.b {
      panic!("Not the same elliptic curve")
    } if let None = self.x {
      return Point{x: None, y: None, a: self.a, b: self.b};
    } if let  None = other.x {
      return Point{x: None, y: None, a: self.a, b: self.b};
    }
    if let None = self.y {
      return Point{x: None, y: None, a: self.a, b: self.b};
    } if let None = other.y {
      return Point{x: None, y: None, a: self.a, b: self.b};
    }
    
    let x1: i32 = self.x.unwrap();
    let x2: i32 = other.x.unwrap();
    
    let y1: i32 = self.y.unwrap();
    let y2: i32 = other.y.unwrap();
    if x1 != x2 {
      let s: i32 = (y2 - y1) / (x2 - x1);
      let x3 = i32::pow(s, 2) - x1 - x2;
      let y3: i32 = s*(x1 - x3) - y1;
      return Point::new(Some(x3), Some(y3), self.a, self.b)
    }
    if self == other && y1 == 0 * x1 {
      return Point::new(None, None, self.a, self.b)
    }
    if self == other {
      let s: i32 = (3 * i32::pow(x1, 2) + self.a) / ( 2 * y1);
      let x3: i32 = i32::pow(s, 2) - 2 * x1;
      let y3: i32 = s * (x1 - x3) - y1;
      return Point::new(Some(x3), Some(y3), self.a, self.b)
    }
    Point {x: None,y: None,a: self.a, b: self.b}
  }
}


#[cfg(test)]
mod point_test {
    use super::*;
    #[test]
    fn point_inside_curve() {
        let pairs = [(-1, 1), (-1, -1), (2, -5), (3, -7), (3, 7), (18, -77)];
        for pair in pairs {
            let (x, y) = pair;
            let _point = Point::new(Some(x), Some(y), 5, 7);
        }
    }
    #[test]
    #[should_panic]
    fn point_outside_curve() {
        let pairs = [(-4, 1), (-3, -1), (55, -5), (1, -7), (3, 9), (111, -77)];
        for pair in pairs {
            let (x, y) = pair;
            let _point = Point::new(Some(x), Some(y), 5, 7);
        }
        let _point = Point::new(Some(-1), Some(-2), 5, 7);
    }
    #[test]
    fn add_same_x_different_y() {
        let point_0 = Point::new(Some(-1), Some(1), 5, 7);
        let point_1 = Point::new(Some(-1), Some(-1), 5, 7);
        let infinity_5_7 = Point::new(None, None, 5, 7);
        let must_be_infinity = point_0 + point_1;
        assert_eq!(must_be_infinity, infinity_5_7);
    }
    #[test]
    fn add_infinite() {
        let inf = None;
        let point_0 = Point::new(inf, inf, 5, 7);
        let point_1 = Point::new(Some(2), Some(5), 5, 7);
        assert_eq!(point_0, (point_0.clone() + point_1));
    }
    #[test]
    fn add_different_x() {
        let point_0 = Point::new(Some(3), Some(7), 5, 7);
        let point_1 = Point::new(Some(-1), Some(-1), 5, 7);
        let result = point_0.clone() + point_1.clone();
        assert_eq!(
            result,
            Point::new(Some(2), Some(-5), 5, 7)
        );
    }
    // Tests taken from the books' repo.
    #[test]
    fn add_equal() {
        let a = Point::new(Some(-1), Some(-1), 5, 7);
        let b = a.clone();
        let result = a + b;
        let expected = Point::new(Some(18), Some(77), 5, 7);
        assert_eq!(expected, result);
    }
    #[test]
    fn not_equal() {
        let a = Point::new(Some(3), Some(-7), 5, 7);
        let b = Point::new(Some(18), Some(77), 5, 7);
        assert!(a != b);
        assert!(a == a);
    }
}
