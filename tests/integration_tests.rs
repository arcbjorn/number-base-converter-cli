use base_converter::{convert_fractional_part, convert_integer_part, format_result, parse_number};

#[test]
fn test_end_to_end_binary_to_decimal() {
    let (int_part, frac_part) = parse_number("1010.101", 2).unwrap();
    let converted_int = convert_integer_part(&int_part, 2, 10);
    let converted_frac = convert_fractional_part(&frac_part, 2, 10, 10);
    let result = format_result(&converted_int, &converted_frac);
    assert_eq!(result, "10.625");
}

#[test]
fn test_end_to_end_hex_to_binary() {
    let (int_part, frac_part) = parse_number("FF.8", 16).unwrap();
    let converted_int = convert_integer_part(&int_part, 16, 2);
    let converted_frac = convert_fractional_part(&frac_part, 16, 2, 10);
    let result = format_result(&converted_int, &converted_frac);
    assert_eq!(result, "11111111.1");
}

#[test]
fn test_end_to_end_decimal_to_hex() {
    let (int_part, frac_part) = parse_number("255.5", 10).unwrap();
    let converted_int = convert_integer_part(&int_part, 10, 16);
    let converted_frac = convert_fractional_part(&frac_part, 10, 16, 10);
    let result = format_result(&converted_int, &converted_frac);
    assert_eq!(result, "FF.8");
}

#[test]
fn test_end_to_end_base36_to_decimal() {
    let (int_part, frac_part) = parse_number("ZZ", 36).unwrap();
    let converted_int = convert_integer_part(&int_part, 36, 10);
    let converted_frac = convert_fractional_part(&frac_part, 36, 10, 10);
    let result = format_result(&converted_int, &converted_frac);
    assert_eq!(result, "1295");
}

#[test]
fn test_end_to_end_zero() {
    let (int_part, frac_part) = parse_number("0", 10).unwrap();
    let converted_int = convert_integer_part(&int_part, 10, 2);
    let converted_frac = convert_fractional_part(&frac_part, 10, 2, 10);
    let result = format_result(&converted_int, &converted_frac);
    assert_eq!(result, "0");
}

#[test]
fn test_end_to_end_fractional_only() {
    let (int_part, frac_part) = parse_number(".25", 10).unwrap();
    let converted_int = convert_integer_part(&int_part, 10, 2);
    let converted_frac = convert_fractional_part(&frac_part, 10, 2, 10);
    let result = format_result(&converted_int, &converted_frac);
    assert_eq!(result, "0.01");
}

#[test]
fn test_end_to_end_large_number() {
    let (int_part, frac_part) = parse_number("12345.6789", 10).unwrap();
    let converted_int = convert_integer_part(&int_part, 10, 16);
    let converted_frac = convert_fractional_part(&frac_part, 10, 16, 8);
    let result = format_result(&converted_int, &converted_frac);
    assert_eq!(result, "3039.ADCC63F1");
}

#[test]
fn test_end_to_end_octal_to_binary() {
    let (int_part, frac_part) = parse_number("777.4", 8).unwrap();
    let converted_int = convert_integer_part(&int_part, 8, 2);
    let converted_frac = convert_fractional_part(&frac_part, 8, 2, 10);
    let result = format_result(&converted_int, &converted_frac);
    assert_eq!(result, "111111111.1");
}

#[test]
fn test_roundtrip_conversion() {
    let original = "CAFE.BABE";
    let (int_part, frac_part) = parse_number(original, 16).unwrap();
    
    let dec_int = convert_integer_part(&int_part, 16, 10);
    let dec_frac = convert_fractional_part(&frac_part, 16, 10, 10);
    
    let back_int = convert_integer_part(&dec_int, 10, 16);
    let back_frac = convert_fractional_part(&dec_frac, 10, 16, 4);
    let result = format_result(&back_int, &back_frac);
    
    assert_eq!(result, "CAFE.BABD");
}

#[test]
fn test_precision_handling() {
    let (_int_part, frac_part) = parse_number("1.333333", 10).unwrap();
    let converted_frac_5 = convert_fractional_part(&frac_part, 10, 3, 5);
    let converted_frac_10 = convert_fractional_part(&frac_part, 10, 3, 10);
    
    assert_eq!(converted_frac_5, vec![0, 2, 2, 2, 2]);
    assert_eq!(converted_frac_10, vec![0, 2, 2, 2, 2, 2, 2, 2, 2, 2]);
}

#[test]
fn test_case_insensitive_parsing() {
    let (int1, frac1) = parse_number("abc.def", 16).unwrap();
    let (int2, frac2) = parse_number("ABC.DEF", 16).unwrap();
    
    assert_eq!(int1, int2);
    assert_eq!(frac1, frac2);
}

#[test]
fn test_error_propagation() {
    assert!(parse_number("123", 1).is_err());
    assert!(parse_number("123", 37).is_err());
    assert!(parse_number("G", 16).is_err());
    assert!(parse_number("1.2.3", 10).is_err());
}