pub fn convert_integer_part(digits: &[u32], from_base: u32, to_base: u32) -> Vec<u32> {
    if digits.is_empty() || (digits.len() == 1 && digits[0] == 0) {
        return vec![0];
    }

    let mut decimal = 0u128;
    let base = from_base as u128;
    
    for &digit in digits {
        decimal = decimal * base + digit as u128;
    }

    if decimal == 0 {
        return vec![0];
    }

    let mut result = Vec::new();
    let target = to_base as u128;
    let mut num = decimal;
    
    while num > 0 {
        result.push((num % target) as u32);
        num /= target;
    }

    result.reverse();
    result
}

pub fn convert_fractional_part(
    digits: &[u32],
    from_base: u32,
    to_base: u32,
    precision: usize,
) -> Vec<u32> {
    if digits.is_empty() {
        return Vec::new();
    }

    let mut decimal_fraction = 0.0;
    let base = from_base as f64;
    
    for (i, &digit) in digits.iter().enumerate() {
        decimal_fraction += digit as f64 / base.powi((i + 1) as i32);
    }

    let mut result = Vec::new();
    let target = to_base as f64;
    let mut fraction = decimal_fraction;
    
    for _ in 0..precision {
        if fraction == 0.0 {
            break;
        }
        
        fraction *= target;
        let digit = fraction.floor() as u32;
        result.push(digit);
        fraction -= digit as f64;
        
        if fraction < 1e-15 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_integer_basic() {
        let result = convert_integer_part(&[1, 0, 1, 0], 2, 10);
        assert_eq!(result, vec![1, 0]);

        let result = convert_integer_part(&[2, 5, 5], 10, 16);
        assert_eq!(result, vec![15, 15]);
    }

    #[test]
    fn test_convert_integer_zero() {
        let result = convert_integer_part(&[0], 10, 2);
        assert_eq!(result, vec![0]);

        let result = convert_integer_part(&[], 10, 2);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_convert_integer_large_numbers() {
        let result = convert_integer_part(&[1, 0, 0, 0, 0, 0, 0], 2, 10);
        assert_eq!(result, vec![6, 4]);

        let result = convert_integer_part(&[9, 9, 9, 9], 10, 2);
        assert_eq!(result, vec![1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1]);
    }

    #[test]
    fn test_convert_integer_base_36() {
        let result = convert_integer_part(&[35, 35], 36, 10);
        assert_eq!(result, vec![1, 2, 9, 5]);

        let result = convert_integer_part(&[1, 2, 9, 5], 10, 36);
        assert_eq!(result, vec![35, 35]);
    }

    #[test]
    fn test_convert_integer_same_base() {
        let result = convert_integer_part(&[1, 2, 3], 10, 10);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_convert_fractional_basic() {
        let result = convert_fractional_part(&[1, 0, 1], 2, 10, 3);
        assert_eq!(result, vec![6, 2, 5]);

        let result = convert_fractional_part(&[5], 10, 2, 10);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_convert_fractional_empty() {
        let result = convert_fractional_part(&[], 10, 2, 10);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_convert_fractional_precision() {
        let result = convert_fractional_part(&[3, 3, 3], 10, 2, 5);
        assert_eq!(result.len(), 5);

        let result = convert_fractional_part(&[3, 3, 3], 10, 2, 20);
        assert!(result.len() <= 20);
    }

    #[test]
    fn test_convert_fractional_terminating() {
        let result = convert_fractional_part(&[5], 10, 2, 100);
        assert_eq!(result, vec![1]);

        let result = convert_fractional_part(&[2, 5], 10, 2, 100);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_convert_fractional_repeating() {
        let result = convert_fractional_part(&[3, 3, 3, 3, 3, 3], 10, 3, 10);
        assert_eq!(result.len(), 10);

        let first_digits: Vec<u32> = result.iter().take(3).cloned().collect();
        assert_eq!(first_digits, vec![0, 2, 2]);
    }

    #[test]
    fn test_convert_fractional_base_conversions() {
        let result = convert_fractional_part(&[8], 16, 10, 10);
        assert_eq!(result, vec![5]);

        let result = convert_fractional_part(&[15], 16, 10, 10);
        assert_eq!(result, vec![9, 3, 7, 5]);
    }
}