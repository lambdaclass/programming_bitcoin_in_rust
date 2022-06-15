use crate::errors::NotPrimeError;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

fn is_prime(number_to_check: u128) -> bool {
    if number_to_check == 1 {
        return false;
    }

    let mut aux = 2;
    while aux * aux <= number_to_check {
        if number_to_check % aux == 0 {
            return false;
        }
        aux += 1;
    }

    true
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteFieldElement<const P: u128> {
    pub value: i128,
}

impl<const P: u128> FiniteFieldElement<P> {
    pub fn new(value: i128) -> Result<Self, NotPrimeError> {
        if !is_prime(P) {
            return Err(NotPrimeError);
        }
        Ok(Self { value })
    }
}

impl<const P: u128> Add<FiniteFieldElement<P>> for FiniteFieldElement<P> {
    type Output = Self;

    fn add(self, other_number: Self) -> Self {
        Self {
            value: (self.value + other_number.value).rem_euclid(P as i128),
        }
    }
}

impl<const P: u128> Sub<FiniteFieldElement<P>> for FiniteFieldElement<P> {
    type Output = Self;

    fn sub(self, other_number: Self) -> Self {
        Self {
            value: (self.value - other_number.value).rem_euclid(P as i128),
        }
    }
}

impl<const P: u128> Mul<FiniteFieldElement<P>> for FiniteFieldElement<P> {
    type Output = Self;

    fn mul(self, other_number: Self) -> Self {
        Self {
            value: (self.value * other_number.value).rem_euclid(P as i128),
        }
    }
}

impl<const P: u128> FiniteFieldElement<P> {
    pub fn pow(&self, n: i128) -> Self {
        let exp = n.rem_euclid((P - 1_u128) as i128);
        let mut value = self.value;
        for _ in 1..exp {
            value *= self.value;
        }
        Self {
            value: value.rem_euclid(P as i128),
        }
    }
}

impl<const P: u128> Div<FiniteFieldElement<P>> for FiniteFieldElement<P> {
    type Output = Self;

    fn div(self, other_number: Self) -> Self {
        self * other_number.pow((P - 2_u128) as i128)
    }
}

impl<const P: u128> MulAssign<FiniteFieldElement<P>> for FiniteFieldElement<P> {
    fn mul_assign(&mut self, other_number: Self) {
        self.value = (self.value * other_number.value).rem_euclid(P as i128);
    }
}

impl<const P: u128> AddAssign<FiniteFieldElement<P>> for FiniteFieldElement<P> {
    fn add_assign(&mut self, other_number: Self) {
        self.value = (self.value + other_number.value).rem_euclid(P as i128);
    }
}

impl<const P: u128> SubAssign<FiniteFieldElement<P>> for FiniteFieldElement<P> {
    fn sub_assign(&mut self, other_number: Self) {
        self.value = (self.value - other_number.value).rem_euclid(P as i128);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn test_can_not_create_field_with_no_prime_order() {
        let _ = FiniteFieldElement::<10>::new(1).unwrap();
    }

    #[test]
    fn test_add_two_finite_field_elements() {
        let first_field_element = FiniteFieldElement::<11>::new(1).unwrap();
        let second_field_element = FiniteFieldElement::<11>::new(20).unwrap();

        assert_eq!(
            first_field_element + second_field_element,
            FiniteFieldElement::<11>::new(10).unwrap()
        );
    }

    #[test]
    fn test_sub_two_finite_field_elements() {
        let first_field_element = FiniteFieldElement::<11>::new(1).unwrap();
        let second_field_element = FiniteFieldElement::<11>::new(20).unwrap();

        assert_eq!(
            first_field_element - second_field_element,
            FiniteFieldElement::<11>::new(3).unwrap()
        );
    }

    #[test]
    fn test_mul_two_finite_field_elements() {
        let first_field_element = FiniteFieldElement::<11>::new(1).unwrap();
        let second_field_element = FiniteFieldElement::<11>::new(20).unwrap();

        assert_eq!(
            first_field_element * second_field_element,
            FiniteFieldElement::<11>::new(9).unwrap()
        );
    }

    #[test]
    fn test_pow_a_finite_field_with_a_number() {
        let first_field_element = FiniteFieldElement::<11>::new(3).unwrap();

        assert_eq!(
            first_field_element.pow(3),
            FiniteFieldElement::<11>::new(5).unwrap()
        );
    }

    #[test]
    fn test_div_two_finite_field_elements() {
        let first_field_element = FiniteFieldElement::<11>::new(1).unwrap();
        let second_field_element = FiniteFieldElement::<11>::new(20).unwrap();

        assert_eq!(
            first_field_element / second_field_element,
            FiniteFieldElement::<11>::new(5).unwrap()
        );
    }
}
