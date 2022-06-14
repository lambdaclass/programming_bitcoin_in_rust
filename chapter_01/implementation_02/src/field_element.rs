use num_bigint::BigInt;
use num_bigint::ToBigInt;
use std::fmt;
use std::ops;

#[derive(Debug, Eq)]
pub struct FieldElement {
    num: BigInt,
    prime: BigInt,
}

impl FieldElement {
    pub fn new(num: BigInt, prime: BigInt) -> Self {
        if num >= prime || num < 0_i32.to_bigint().unwrap() {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        Self { num, prime }
    }

    pub fn pow(&self, exponent: BigInt) -> Self {
        let positive_exponent = exponent.rem_euclid(self.prime.clone() - 1);
        let num = self.num.modpow(&positive_exponent, &self.prime);
        Self {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl ops::Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num + rhs.num).rem_euclid(self.prime.clone());
        Self {
            num,
            prime: self.prime,
        }
    }
}

impl ops::Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }
        let num = (self.num - rhs.num).rem_euclid(self.prime.clone());
        Self {
            num,
            prime: self.prime,
        }
    }
}

impl ops::Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        let num = (self.num * rhs.num).rem_euclid(self.prime.clone());
        Self {
            num,
            prime: self.prime,
        }
    }
}

impl ops::Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot divide two numbers in different Fields");
        }
        // a / b == a * b.pow(p - 2)
        let exponent = self.prime.clone() - 2_i32.to_bigint().unwrap();
        let rhs_factor = rhs.num.modpow(&exponent, &self.prime);
        let num = (self.num * rhs_factor) % self.prime.clone();
        Self {
            num,
            prime: self.prime,
        }
    }
}

trait RemEuclid {
    fn rem_euclid(&self, rhs: Self) -> Self;
}

impl RemEuclid for BigInt {
    fn rem_euclid(&self, rhs: Self) -> Self {
        self.modpow(&1_i32.to_bigint().unwrap(), &rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_field_elements() {
        let prime = 13_i32.to_bigint().unwrap();
        let a_num = 7_i32.to_bigint().unwrap();
        let b_num = 12_i32.to_bigint().unwrap();
        let c_num = 6_i32.to_bigint().unwrap();
        let a = FieldElement::new(a_num, prime.clone());
        let b = FieldElement::new(b_num, prime.clone());
        let c = FieldElement::new(c_num, prime);

        assert_eq!(a + b, c);
    }

    #[test]
    fn substract_two_field_elements() {
        let prime = 19_i32.to_bigint().unwrap();
        let a_num = 6_i32.to_bigint().unwrap();
        let b_num = 13_i32.to_bigint().unwrap();
        let c_num = 12_i32.to_bigint().unwrap();
        let a = FieldElement::new(a_num, prime.clone());
        let b = FieldElement::new(b_num, prime.clone());
        let c = FieldElement::new(c_num, prime);

        assert_eq!(a - b, c);
    }

    #[test]
    fn multiply_two_field_elements() {
        let prime = 13_i32.to_bigint().unwrap();
        let a_num = 3_i32.to_bigint().unwrap();
        let b_num = 12_i32.to_bigint().unwrap();
        let c_num = 10_i32.to_bigint().unwrap();
        let a = FieldElement::new(a_num, prime.clone());
        let b = FieldElement::new(b_num, prime.clone());
        let c = FieldElement::new(c_num, prime);

        assert_eq!(a * b, c);
    }

    #[test]
    fn power_a_field_element_to_a_positive_exponent() {
        let prime = 13_i32.to_bigint().unwrap();
        let a_num = 3_i32.to_bigint().unwrap();
        let b_num = 1_i32.to_bigint().unwrap();
        let a = FieldElement::new(a_num, prime.clone());
        let b = FieldElement::new(b_num, prime);
        let exponent = 3_i32.to_bigint().unwrap();

        assert_eq!(a.pow(exponent), b);
    }

    #[test]
    fn divide_two_field_elements() {
        let prime = 19_i32.to_bigint().unwrap();
        let a_num = 2_i32.to_bigint().unwrap();
        let b_num = 7_i32.to_bigint().unwrap();
        let c_num = 3_i32.to_bigint().unwrap();
        let a = FieldElement::new(a_num, prime.clone());
        let b = FieldElement::new(b_num, prime.clone());
        let c = FieldElement::new(c_num, prime);

        assert_eq!(a / b, c);
    }

    #[test]
    fn power_a_field_element_to_a_negative_exponent() {
        let prime = 13_i32.to_bigint().unwrap();
        let a_num = 7_i32.to_bigint().unwrap();
        let b_num = 8_i32.to_bigint().unwrap();
        let a = FieldElement::new(a_num, prime.clone());
        let b = FieldElement::new(b_num, prime);
        let exponent = -3_i32.to_bigint().unwrap();

        assert_eq!(a.pow(exponent), b);
    }
}
