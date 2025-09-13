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
    fn test_char_to_digit() {
        assert_eq!(char_to_digit('0', 10).unwrap(), 0);
        assert_eq!(char_to_digit('A', 16).unwrap(), 10);
        assert!(char_to_digit('G', 16).is_err());
    }

    #[test]
    fn test_parse_number() {
        let (int_part, frac_part) = parse_number("1010.101", 2).unwrap();
        assert_eq!(int_part, vec![1, 0, 1, 0]);
        assert_eq!(frac_part, vec![1, 0, 1]);
    }
}