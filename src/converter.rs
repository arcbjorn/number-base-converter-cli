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

