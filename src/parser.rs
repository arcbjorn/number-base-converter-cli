fn char_to_digit(c: char, base: u32) -> Result<u32, String> {
    let digit = if c.is_ascii_digit() {
        c.to_digit(10).unwrap()
    } else if c.is_ascii_alphabetic() {
        c.to_ascii_uppercase() as u32 - 'A' as u32 + 10
    } else {
        return Err(format!("Invalid character: '{}'", c));
    };

    if digit >= base {
        return Err(format!("Digit '{}' is invalid for base {}", c, base));
    }

    Ok(digit)
}

pub fn parse_number(value: &str, base: u32) -> Result<(Vec<u32>, Vec<u32>), String> {
    if base < 2 || base > 36 {
        return Err(format!("Base must be between 2 and 36, got {}", base));
    }

    let parts: Vec<&str> = value.split('.').collect();
    if parts.len() > 2 {
        return Err("Invalid number format: multiple decimal points".to_string());
    }

    let integer_part = if parts[0].is_empty() {
        if parts.len() == 1 {
            return Err("Empty number".to_string());
        }
        vec![0]
    } else {
        let mut result = Vec::new();
        for c in parts[0].chars() {
            result.push(char_to_digit(c, base)?);
        }
        result
    };

    let fractional_part = if parts.len() == 2 {
        let mut result = Vec::new();
        for c in parts[1].chars() {
            result.push(char_to_digit(c, base)?);
        }
        result
    } else {
        Vec::new()
    };

    Ok((integer_part, fractional_part))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_digit_valid() {
        assert_eq!(char_to_digit('0', 10).unwrap(), 0);
        assert_eq!(char_to_digit('9', 10).unwrap(), 9);
        assert_eq!(char_to_digit('A', 16).unwrap(), 10);
        assert_eq!(char_to_digit('F', 16).unwrap(), 15);
        assert_eq!(char_to_digit('Z', 36).unwrap(), 35);
        assert_eq!(char_to_digit('a', 16).unwrap(), 10);
        assert_eq!(char_to_digit('z', 36).unwrap(), 35);
    }

    #[test]
    fn test_char_to_digit_invalid() {
        assert!(char_to_digit('G', 16).is_err());
        assert!(char_to_digit('9', 8).is_err());
        assert!(char_to_digit('2', 2).is_err());
        assert!(char_to_digit('@', 36).is_err());
        assert!(char_to_digit('!', 10).is_err());
        assert!(char_to_digit(' ', 10).is_err());
    }

    #[test]
    fn test_parse_number_basic() {
        let (int_part, frac_part) = parse_number("1010.101", 2).unwrap();
        assert_eq!(int_part, vec![1, 0, 1, 0]);
        assert_eq!(frac_part, vec![1, 0, 1]);

        let (int_part, frac_part) = parse_number("FF.8", 16).unwrap();
        assert_eq!(int_part, vec![15, 15]);
        assert_eq!(frac_part, vec![8]);
    }

    #[test]
    fn test_parse_number_integer_only() {
        let (int_part, frac_part) = parse_number("123", 10).unwrap();
        assert_eq!(int_part, vec![1, 2, 3]);
        assert_eq!(frac_part, vec![]);
    }

    #[test]
    fn test_parse_number_fractional_only() {
        let (int_part, frac_part) = parse_number(".123", 10).unwrap();
        assert_eq!(int_part, vec![0]);
        assert_eq!(frac_part, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_number_zero() {
        let (int_part, frac_part) = parse_number("0", 10).unwrap();
        assert_eq!(int_part, vec![0]);
        assert_eq!(frac_part, vec![]);

        let (int_part, frac_part) = parse_number("0.0", 10).unwrap();
        assert_eq!(int_part, vec![0]);
        assert_eq!(frac_part, vec![0]);
    }

    #[test]
    fn test_parse_number_base_36() {
        let (int_part, frac_part) = parse_number("ZZ.Z", 36).unwrap();
        assert_eq!(int_part, vec![35, 35]);
        assert_eq!(frac_part, vec![35]);
    }

    #[test]
    fn test_parse_number_mixed_case() {
        let (int_part, frac_part) = parse_number("aBc.DeF", 16).unwrap();
        assert_eq!(int_part, vec![10, 11, 12]);
        assert_eq!(frac_part, vec![13, 14, 15]);
    }

    #[test]
    fn test_parse_number_invalid_base() {
        assert!(parse_number("123", 1).is_err());
        assert!(parse_number("123", 37).is_err());
        assert!(parse_number("123", 0).is_err());
    }

    #[test]
    fn test_parse_number_multiple_decimal_points() {
        assert!(parse_number("1.2.3", 10).is_err());
        assert!(parse_number("...", 10).is_err());
        assert!(parse_number("1..2", 10).is_err());
    }

    #[test]
    fn test_parse_number_invalid_characters() {
        assert!(parse_number("1G", 16).is_err());
        assert!(parse_number("12@", 10).is_err());
        assert!(parse_number("A.B", 10).is_err());
        assert!(parse_number("1 2", 10).is_err());
    }

    #[test]
    fn test_parse_number_empty() {
        assert!(parse_number("", 10).is_err());
        assert!(parse_number(".", 10).is_ok());
    }

    #[test]
    fn test_parse_number_leading_zeros() {
        let (int_part, frac_part) = parse_number("001.100", 10).unwrap();
        assert_eq!(int_part, vec![0, 0, 1]);
        assert_eq!(frac_part, vec![1, 0, 0]);
    }

    #[test]
    fn test_parse_number_edge_bases() {
        let (int_part, _) = parse_number("10", 2).unwrap();
        assert_eq!(int_part, vec![1, 0]);

        let (int_part, _) = parse_number("ZZ", 36).unwrap();
        assert_eq!(int_part, vec![35, 35]);
    }
}