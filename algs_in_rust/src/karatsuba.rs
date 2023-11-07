use std::cmp;

/// Count the number of digits in a u32; as an example, if `num = 12345`, `num_digits(num) = 5`.
pub(crate) fn num_digits(mut num: u32) -> u8 {
    if num == 0 {
        return 1;
    }

    let mut count = 0;
    while num != 0 {
        num /= 10;
        count += 1;
    }
    count
}

/// Split a number at the nth digit and return the digits before and after that digit;
/// meaning, if `num` is a number with digits `a4,a3,a2,a1,a0`, calling `split_num(num, i)`
/// will return a tuple where the first number in the tuple is every digit with a subscript
/// greater than or equal to i, and the second number in the tuple is every digit with a
/// subscript less than i.
pub(crate) fn split_num(num: u32, digit: u8) -> (u32, u32) {
    let left = num / 10_u32.pow(digit as u32);
    let right = num % 10_u32.pow(digit as u32);
    (left, right)
}

/// Given two integers, multiply them using Karatsuba's algorithm for integer multiplication;
/// also returns and uses the total number of single-digit integer multiplications.
/// `a` and `b` are the two digits being multiplied together, while `acc` is used to
/// keep track of the number of single-digit multiplications.
pub fn karatsuba(a: u32, b: u32, acc: u8) -> (u32, u8) {
    if a < 10 || b < 10 {
        (
            a * b,
            acc + (if a < 10 && b < 10 {
                1
            } else if a < 10 {
                num_digits(b)
            } else {
                // b < 10
                num_digits(a)
            }),
        )
    } else {
        let split = cmp::max(num_digits(a), num_digits(b)) / 2;
        let (a1, a0) = split_num(a, split);
        let (b1, b0) = split_num(b, split);
        let (k1, acc1) = karatsuba(a1, b1, acc);
        let (k2, acc2) = karatsuba(a1 + a0, b1 + b0, acc1);
        let (k3, acc3) = karatsuba(a0, b0, acc2);
        let result =
            10_u32.pow(2 * (split as u32)) * k1 + 10_u32.pow(split as u32) * (k2 - k3 - k1) + k3;
        (result, acc3)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_num_digits() {
        assert_eq!(5, num_digits(35103));
        assert_eq!(1, num_digits(0));
        assert_eq!(3, num_digits(132));
        assert_eq!(9, num_digits(222222222));
        assert_eq!(4, num_digits(9999));
    }

    #[test]
    fn test_split_number() {
        assert_eq!(split_num(12345, 3), (12, 345));
        assert_eq!(split_num(335, 3), (0, 335));
        assert_eq!(split_num(5738294, 4), (573, 8294));
        assert_eq!(split_num(6543210, num_digits(6543210) / 2), (6543, 210));
    }

    #[test]
    fn test_karatsuba() {
        assert_eq!(karatsuba(12, 12, 0), (144, 3));
        assert_eq!(karatsuba(98, 57, 0), (98 * 57, 5));
        assert_eq!(karatsuba(1354, 4021, 0), (1354 * 4021, 10));
        assert_eq!(karatsuba(4851, 2723, 0), (4851 * 2723, 11));
    }
}
