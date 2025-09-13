use base_converter::parser::parse_number;

#[test]
fn test_char_to_digit_valid() {
    let (int_part, _) = parse_number("0", 10).unwrap();
    assert_eq!(int_part[0], 0);
    let (int_part, _) = parse_number("9", 10).unwrap();
    assert_eq!(int_part[0], 9);
    let (int_part, _) = parse_number("A", 16).unwrap();
    assert_eq!(int_part[0], 10);
    let (int_part, _) = parse_number("F", 16).unwrap();
    assert_eq!(int_part[0], 15);
    let (int_part, _) = parse_number("Z", 36).unwrap();
    assert_eq!(int_part[0], 35);
    let (int_part, _) = parse_number("a", 16).unwrap();
    assert_eq!(int_part[0], 10);
    let (int_part, _) = parse_number("z", 36).unwrap();
    assert_eq!(int_part[0], 35);
}

#[test]
fn test_char_to_digit_invalid() {
    assert!(parse_number("G", 16).is_err());
    assert!(parse_number("9", 8).is_err());
    assert!(parse_number("2", 2).is_err());
    assert!(parse_number("@", 36).is_err());
    assert!(parse_number("!", 10).is_err());
    assert!(parse_number(" ", 10).is_err());
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