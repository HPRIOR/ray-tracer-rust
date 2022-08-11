use core::panic;

trait ToU32 {
    fn to_u32(&self) -> u32;
}

impl ToU32 for char {
    fn to_u32(&self) -> u32 {
        *self as u32 - '0' as u32
    }
}

fn round_from_next_num(this: u32, next: u32) -> u32 {
    if next >= 5 {
        this + 1
    } else {
        this
    }
}

fn get_decimal(inp: f64) -> i32 {
    let inp_i32_str: String = inp.to_string().split(".").take(1).collect();
    inp_i32_str.parse().unwrap()
}

fn compare_integer(a: f64, b: f64) {
    let a_i32 = get_decimal(a);
    let b_i32 = get_decimal(b);
    if a_i32 != b_i32 {
        panic!("Decimal not equal: a: {} b: {}", a_i32, b_i32);
    }
}

fn compare(a: f64, b: f64, precision: usize) {
    compare_integer(a, b);

    let fractional_str_a = a.to_string();
    let fractional_str_b = b.to_string();

    let a_split: Vec<&str> = fractional_str_a.split(".").collect();
    let b_split: Vec<&str> = fractional_str_b.split(".").collect();

    if a_split.len() == 1 || b_split.len() == 1 {
        return;
    }

    let fractional_a = a_split.last().unwrap();
    let fractional_b = b_split.last().unwrap();

    let chars: Vec<(char, char)> = fractional_a.chars().zip(fractional_b.chars()).collect();

    for (i, (a_char, b_char)) in chars.iter().enumerate() {
        if i > precision {
            return;
        }
        if i == precision {
            if let Some((next_a, next_b)) = chars.get(precision + 1) {
                let (this_a_num, this_b_num) = (a_char.to_u32(), b_char.to_u32());
                let (next_a_num, next_b_num) = (next_a.to_u32(), next_b.to_u32());
                let (rounded_a, rounded_b) = (
                    round_from_next_num(this_a_num, next_a_num),
                    round_from_next_num(this_b_num, next_b_num),
                );

                if rounded_a != rounded_b {
                    panic!("Numbers are not equal when rounded; a:{} and b:{} do not match at index {}.\nRounded to {} {} respectively", a_char, b_char, i, rounded_a, rounded_b)
                }

                println!("{} ~= {}", a, b);
                return;
            } else {
                println!("Could not round, precision to large")
            }
        }

        if a_char.to_u32() != b_char.to_u32() {
            panic!(
                "Numbers are not equal; a:{} and b:{} do not match at index {}",
                a_char, b_char, i
            )
        }
    }
}

pub fn tup_approx_eq(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64), precision: usize) {
    compare(a.0, b.0, precision);
    compare(a.1, b.1, precision);
    compare(a.2, b.2, precision);
    compare(a.3, b.3, precision);
}

#[cfg(test)]
mod tests {

    use num_traits::Float;

    use super::{compare, tup_approx_eq};

    #[test]
    fn two_negative_chars_do_not_panic() {
        compare(-1.0001, -1.0001, 2)
    }

    #[test]
    #[should_panic]
    fn negative_and_positive_should_panic() {
        compare(-1.0001, 1.0001, 2)
    }

    #[test]
    fn two_positive_numbers_should_not_panic() {
        compare(1.0001, 1.0001, 2)
    }

    #[test]
    #[should_panic]
    fn different_decimals_panic() {
        compare(1.0001, 10.001, 2)
    }

    #[test]
    fn numbers_are_rounded_correctly() {
        compare(1.025, 1.034, 1);
    }

    #[test]
    #[should_panic]
    fn numbers_will_not_be_rounded_with_high_precision() {
        compare(1.025, 1.034, 4);
    }

    #[test]
    #[should_panic]
    fn will_behave_when_precision_equals_last_digit() {
        compare(1.025, 1.034, 2);
    }

    #[test]
    fn will_behave_correctly_with_zero_precision() {
        compare(1.045, 1.034, 0);
    }

    #[test]
    #[should_panic]
    fn will_panic_with_zero_precision() {
        compare(1.125, 1.154, 0);
    }

    #[test]
    fn can_compare_two_tuples() {
        let a = (0.0, 0.70710677, 0.7071068, 1.0);
        let b = (0.0, (2.0).sqrt() / 2.0, (2.0).sqrt() / 2.0, 1.0);
        tup_approx_eq(a, b, 5);
    }

    #[test]
    fn minus_one_can_be_comared(){
        compare(-1.0, -1.0, 5);
    }
}
