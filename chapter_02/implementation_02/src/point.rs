use num_bigint::BigInt;
use std::fmt;
use std::ops;

#[derive(Clone, Debug, Eq)]
pub struct Point {
    x: Option<BigInt>,
    y: Option<BigInt>,
    a: BigInt,
    b: BigInt,
}

impl Point {
    pub fn new(x: Option<BigInt>, y: Option<BigInt>, a: BigInt, b: BigInt) -> Self {
        match (x.clone(), y.clone()) {
            (Some(x_num), Some(y_num)) => {
                if y_num.pow(2) != x_num.pow(3) + a.clone() * x_num.clone() + b.clone() {
                    panic!("({}, {}) is not on the curve", x_num, y_num);
                }
            }
            (Some(x_num), None) => {
                panic!("({}, None) is not valid", x_num);
            }
            (None, Some(y_num)) => {
                panic!("(None, {}) is not valid", y_num);
            }
            (None, None) => {}
        }
        Self { x, y, a, b }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.x.clone(), self.y.clone()) {
            (Some(x_num), Some(y_num)) => {
                write!(f, "Point({},{})_{}_{}", x_num, y_num, self.a, self.b)
            }
            (None, None) => write!(f, "Point(infinity)_{}_{}", self.a, self.b),
            _ => {
                panic!("This shouldn't happen");
            }
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("Points {}, {} are not on the same curve", self, rhs);
        }
        match (
            (self.x.clone(), self.y.clone()),
            (rhs.x.clone(), rhs.y.clone()),
        ) {
            ((None, _), (Some(_), _)) => rhs,
            ((Some(_), _), (None, _)) => self,
            ((Some(self_x), Some(self_y)), (Some(rhs_x), Some(rhs_y)))
                if self_x == rhs_x && self_y == -rhs_y.clone() =>
            {
                Self {
                    x: None,
                    y: None,
                    a: self.a,
                    b: self.b,
                }
            }
            ((Some(self_x), Some(self_y)), (Some(rhs_x), Some(rhs_y))) if self_x != rhs_x => {
                let slope = (rhs_y - self_y.clone()) / (rhs_x.clone() - self_x.clone());
                println!("slope: {}", slope.clone());
                let result_x = slope.clone() * slope.clone() - self_x.clone() - rhs_x;
                let result_y = slope * (self_x - result_x.clone()) - self_y;

                Self {
                    x: Some(result_x),
                    y: Some(result_y),
                    a: self.a,
                    b: self.b,
                }
            }
            // Other cases, TODO
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::ToBigInt;

    use super::*;

    #[test]
    #[should_panic]
    fn create_a_point_that_is_not_in_the_curve() {
        let p1_x = Some(-1_i32.to_bigint().unwrap());
        let p1_y = Some(-1_i32.to_bigint().unwrap());
        let a = 5_i32.to_bigint().unwrap();
        let b = 7_i32.to_bigint().unwrap();
        let p2_x = Some(-1_i32.to_bigint().unwrap());
        let p2_y = Some(-2_i32.to_bigint().unwrap());
        Point::new(p1_x, p1_y, a.clone(), b.clone());
        Point::new(p2_x, p2_y, a.clone(), b.clone());
    }
    #[test]
    fn compare_two_points() {
        let p1_x = Some(-1_i32.to_bigint().unwrap());
        let p1_y = Some(-1_i32.to_bigint().unwrap());
        let a = 5_i32.to_bigint().unwrap();
        let b = 7_i32.to_bigint().unwrap();
        let p2_x = Some(-1_i32.to_bigint().unwrap());
        let p2_y = Some(-1_i32.to_bigint().unwrap());
        let p1 = Point::new(p1_x, p1_y, a.clone(), b.clone());
        let p2 = Point::new(p2_x, p2_y, a.clone(), b.clone());

        assert_eq!(p1, p2);
    }

    #[test]
    fn add_two_points_with_the_same_x() {
        let p1_x = Some(-1_i32.to_bigint().unwrap());
        let p1_y = Some(-1_i32.to_bigint().unwrap());
        let a = 5_i32.to_bigint().unwrap();
        let b = 7_i32.to_bigint().unwrap();
        let p2_x = Some(-1_i32.to_bigint().unwrap());
        let p2_y = Some(1_i32.to_bigint().unwrap());
        let p1 = Point::new(p1_x, p1_y, a.clone(), b.clone());
        let p2 = Point::new(p2_x, p2_y, a.clone(), b.clone());
        let inf = Point::new(None, None, a.clone(), b.clone());

        assert_eq!(p1.clone() + inf.clone(), p1);
        assert_eq!(inf.clone() + p2.clone(), p2);
        assert_eq!(p1 + p2, inf);
    }
    #[test]
    fn add_two_points_with_different_x() {
        let p1_x = Some(2_i32.to_bigint().unwrap());
        let p1_y = Some(5_i32.to_bigint().unwrap());
        let a = 5_i32.to_bigint().unwrap();
        let b = 7_i32.to_bigint().unwrap();
        let p2_x = Some(-1_i32.to_bigint().unwrap());
        let p2_y = Some(-1_i32.to_bigint().unwrap());
        let p3_x = Some(3_i32.to_bigint().unwrap());
        let p3_y = Some(-7_i32.to_bigint().unwrap());
        let p1 = Point::new(p1_x, p1_y, a.clone(), b.clone());
        let p2 = Point::new(p2_x, p2_y, a.clone(), b.clone());
        let p3 = Point::new(p3_x, p3_y, a.clone(), b.clone());

        assert_eq!(p1 + p2, p3);
    }
}
