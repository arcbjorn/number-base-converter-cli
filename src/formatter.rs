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

