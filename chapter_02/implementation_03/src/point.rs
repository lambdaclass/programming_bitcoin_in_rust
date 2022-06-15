use anyhow::ensure;
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub enum Point<const A: i64, const B: i64> {
    Point(i64, i64),
    Infinity
}

impl<const A: i64, const B: i64> Point<A, B> {
    pub fn new_point(x: i64, y: i64) -> Result<Self, anyhow::Error> {
        ensure!(y.pow(2) == (x.pow(3) + A * x + B), "Point is not on the curve");
        Ok(Point::<A, B>::Point(x, y))
    }

    pub fn new_infinity() -> Self {
        Point::<A, B>::Infinity
    }
}

impl<const A: i64, const B: i64> PartialEq for Point<A, B> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Infinity, Self::Infinity) => true,
            (Self::Point(x1, y1), Self::Point(x2, y2)) => x1 == x2 && y1 == y2,
            _ => false,
        }
    }
}

impl<const A: i64, const B: i64> Add<Point<A, B>> for Point<A, B> {
    type Output = Self;
    fn add(self, other: Point<A, B>) -> Self {
        match (self, other) {
            (Self::Infinity, Self::Infinity) => Self::new_infinity(),
            (Self::Infinity, Self::Point(_, _)) => other,
            (Self::Point(_, _), Self::Infinity) => self,
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 == x2 && y1 != y2 => Self::new_infinity(),
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 == x2 && (y1 == 0 || y2 == 0) => Self::new_infinity(),
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 == x2 && y1 == y2 => {
                let slope = (3 * x1.pow(2) + A) / (2 * y1);
                let x3 = slope.pow(2) - (2 * x1);
                let y3 = slope * (x1 - x3) - y1;
                Self::new_point(x3, y3).unwrap()
            },
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 != x2 => {
                let slope = (y2 - y1) / (x2 - x1);
                let x3 = slope.pow(2) - (x1 + x2);
                let y3 = slope * (x1 - x3) - y1;
                Self::new_point(x3, y3).unwrap()
            },
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;

    #[test]
    fn test00_create_valid_point() {
        assert!(Point::<5, 7>::new_point(-1, -1).is_ok());
    }

    #[test]
    fn test01_create_invalid_point() {
        assert!(Point::<5, 7>::new_point(-1, -2).is_err());
    }

    #[test]
    fn test02_points_with_same_cords_are_equal() {
        let point1 = Point::<5, 7>::new_point(-1, -1).unwrap();
        let point2 = Point::<5, 7>::new_point(-1, -1).unwrap();

        assert_eq!(point1, point2);
    }

    #[test]
    fn test03_points_with_different_cords_are_different() {
        let point1 = Point::<5, 7>::new_point(-1, -1).unwrap();
        let point2 = Point::<5, 7>::new_point(18, 77).unwrap();

        assert_ne!(point1, point2);
    }

    #[test]
    fn test04_sum_inifity_and_another_point_returns_the_point() {
        let infinity = Point::<5, 7>::new_infinity();
        let point = Point::<5, 7>::new_point(-1, -1).unwrap();

        assert_eq!(point + infinity, point);
        assert_eq!(infinity + point, point);
    }

    #[test]
    fn test05_two_different_points_from_the_same_curve_can_be_added() {
        let point1 = Point::<5, 7>::new_point(2, 5).unwrap();
        let point2 = Point::<5, 7>::new_point(-1, -1).unwrap();
        let expected_point3 = Point::<5, 7>::new_point(3, -7).unwrap();

        assert_eq!(point1 + point2, expected_point3);
        assert_eq!(point2 + point1, expected_point3);
    }

    #[test]
    fn test06_adding_the_same_point_results_infinity() {
        let point1 = Point::<5, 7>::new_point(-1, -1).unwrap();
        let point2 = Point::<5, 7>::new_point(-1, 1).unwrap();

        let expected_point3 = Point::<5, 7>::new_infinity();

        assert_eq!(point1 + point2, expected_point3);
    }
}
