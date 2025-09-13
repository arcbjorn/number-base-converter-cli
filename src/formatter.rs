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
    fn test_digit_to_char() {
        assert_eq!(digit_to_char(0), '0');
        assert_eq!(digit_to_char(9), '9');
        assert_eq!(digit_to_char(10), 'A');
        assert_eq!(digit_to_char(15), 'F');
    }

    #[test]
    fn test_format_result() {
        assert_eq!(format_result(&[1, 0], &[]), "10");
        assert_eq!(format_result(&[15, 15], &[8]), "FF.8");
        assert_eq!(format_result(&[], &[5]), "0.5");
        assert_eq!(format_result(&[0], &[]), "0");
    }
}