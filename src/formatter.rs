fn digit_to_char(digit: u32) -> char {
    if digit < 10 {
        (digit as u8 + b'0') as char
    } else {
        ((digit - 10) as u8 + b'A') as char
    }
}

pub fn format_result(integer_digits: &[u32], fractional_digits: &[u32]) -> String {
    let mut result = String::new();
    
    if integer_digits.is_empty() {
        result.push('0');
    } else {
        for &digit in integer_digits {
            result.push(digit_to_char(digit));
        }
    }
    
    if !fractional_digits.is_empty() {
        result.push('.');
        for &digit in fractional_digits {
            result.push(digit_to_char(digit));
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_to_char_numeric() {
        assert_eq!(digit_to_char(0), '0');
        assert_eq!(digit_to_char(1), '1');
        assert_eq!(digit_to_char(9), '9');
    }

    #[test]
    fn test_digit_to_char_alphabetic() {
        assert_eq!(digit_to_char(10), 'A');
        assert_eq!(digit_to_char(11), 'B');
        assert_eq!(digit_to_char(15), 'F');
        assert_eq!(digit_to_char(25), 'P');
        assert_eq!(digit_to_char(35), 'Z');
    }

    #[test]
    fn test_format_result_integer_only() {
        assert_eq!(format_result(&[1, 0], &[]), "10");
        assert_eq!(format_result(&[0], &[]), "0");
        assert_eq!(format_result(&[1, 2, 3], &[]), "123");
        assert_eq!(format_result(&[35, 35], &[]), "ZZ");
    }

    #[test]
    fn test_format_result_fractional_only() {
        assert_eq!(format_result(&[], &[5]), "0.5");
        assert_eq!(format_result(&[], &[1, 2, 3]), "0.123");
        assert_eq!(format_result(&[], &[0]), "0.0");
        assert_eq!(format_result(&[], &[15, 14, 13]), "0.FED");
    }

    #[test]
    fn test_format_result_mixed() {
        assert_eq!(format_result(&[15, 15], &[8]), "FF.8");
        assert_eq!(format_result(&[1, 0], &[5]), "10.5");
        assert_eq!(format_result(&[2, 5, 5], &[7, 5]), "255.75");
        assert_eq!(format_result(&[35], &[35, 0, 35]), "Z.Z0Z");
    }

    #[test]
    fn test_format_result_empty_integer() {
        assert_eq!(format_result(&[], &[]), "0");
        assert_eq!(format_result(&[], &[1, 0, 1]), "0.101");
    }

    #[test]
    fn test_format_result_zero() {
        assert_eq!(format_result(&[0], &[]), "0");
        assert_eq!(format_result(&[0], &[0]), "0.0");
        assert_eq!(format_result(&[0, 0], &[0, 0]), "00.00");
    }

    #[test]
    fn test_format_result_single_digits() {
        assert_eq!(format_result(&[1], &[]), "1");
        assert_eq!(format_result(&[9], &[]), "9");
        assert_eq!(format_result(&[10], &[]), "A");
        assert_eq!(format_result(&[35], &[]), "Z");
    }

    #[test]
    fn test_format_result_long_numbers() {
        let long_int = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let long_frac = vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(format_result(&long_int, &long_frac), "1234567890.0987654321");
    }

    #[test]
    fn test_format_result_hex_examples() {
        assert_eq!(format_result(&[13, 14, 10, 13], &[11, 14, 14, 15]), "DEAD.BEEF");
        assert_eq!(format_result(&[12, 10, 15, 14], &[11, 10, 11, 14]), "CAFE.BABE");
    }

    #[test]
    fn test_format_result_base36_examples() {
        assert_eq!(format_result(&[7, 4, 11, 11, 14], &[]), "74BBE");
        assert_eq!(format_result(&[22, 4, 7, 11, 3], &[]), "M47B3");
    }
}