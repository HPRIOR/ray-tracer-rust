use core::panic;

use crate::geometry::vector::Tup;

trait ToU32 {
    fn to_u32(&self) -> u32;
}

impl ToU32 for char {
    fn to_u32(&self) -> u32 {
        *self as u32 - '0' as u32
    }
}

pub trait ApproxEq {
    type Type;

    fn approx_eq(self, other: Self::Type);
}

impl ApproxEq for Tup {
    type Type = Self;

    fn approx_eq(self, other: Self::Type) {
        tup_approx_eq(self, other);
    }
}

fn compare(a: f64, b: f64) -> Result<(), String> {
    let epsilon = 0.00001;
    let diff = (a - b).abs();
    if diff < epsilon {
        Ok(())
    } else {
        Err(format!(
            "Diffirence between {} and {} is greater than {}",
            a, b, epsilon
        ))
    }
}

fn tup_approx_eq(a: Tup, b: Tup) {
    let compare_list = vec![
        compare(a.0, b.0),
        compare(a.1, b.1),
        compare(a.2, b.2),
        compare(a.3, b.3),
    ];
    let errors: Vec<String> = compare_list.into_iter().filter_map(|x| x.err()).collect();
    if errors.len() > 0 {
        let error_msg = errors.join("\n");
        panic!("{}", error_msg);
    };
}

#[cfg(test)]
mod tests {

    use super::{compare, tup_approx_eq, ApproxEq};

    #[test]
    fn two_negative_chars_do_not_panic() {
        let sut = compare(-1.0001, -1.0001);
        assert!(sut.is_ok());
    }

    #[test]
    fn negative_and_positive_should_return_error() {
        let sut = compare(-1.0001, 1.0001);
        assert!(sut.is_err());
    }

    #[test]
    fn two_positive_numbers_should_not_panic() {
        let sut = compare(1.0001, 1.0001);
        assert!(sut.is_ok());
    }

    #[test]
    fn different_decimals_panic() {
        let sut = compare(1.0001, 10.001);
        assert!(sut.is_err());
    }

    #[test]
    fn can_compare_two_tuples() {
        let a = (0.0, 0.70710677, 0.7071068, 1.0);
        let b = (0.0, (2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0, 1.0);
        tup_approx_eq(a, b);
    }

    #[test]
    fn can_compare_two_tuples_with_trait_syntax() {
        let a = (0.0, 0.70710677, 0.7071068, 1.0);
        let b = (0.0, (2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0, 1.0);
        a.approx_eq(b);
    }
    #[test]
    #[should_panic]
    fn can_compare_two_tuples_and_panic() {
        let a = (0.0, 0.70710677, 1.0, 1.0);
        let b = (0.0, (2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0, 1.0);
        tup_approx_eq(a, b);
    }

    #[test]
    fn minus_one_can_be_compared() {
        let sut = compare(-1.0, -1.0);
        assert!(sut.is_ok());
    }
}