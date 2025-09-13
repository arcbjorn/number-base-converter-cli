# Base Converter

A Rust CLI tool for converting numbers between different base systems (2-36), with support for fractional values.

## Installation

```bash
cargo build --release
```

## Usage

### Command Line Mode

```bash
./target/release/base-converter --value <NUMBER> --from-base <BASE> --to-base <BASE> [--precision <DIGITS>]
```

### Interactive Mode

```bash
./target/release/base-converter --interactive
```

### Options

- `-v, --value` - Number to convert (e.g., "1010.101", "FF.8")
- `-s, --from-base` - Source base (2-36)
- `-t, --to-base` - Target base (2-36)
- `-p, --precision` - Decimal places for fractions (default: 10)
- `-i, --interactive` - Run in interactive mode

### Examples

```bash
# Binary to decimal
./target/release/base-converter --value 1010.101 --from-base 2 --to-base 10

# Hexadecimal to binary
./target/release/base-converter --value FF.8 --from-base 16 --to-base 2

# Decimal to hex with 8 decimal places
./target/release/base-converter --value 3.14159 --from-base 10 --to-base 16 -p 8

# Interactive mode
./target/release/base-converter --interactive
```

## Features

- Integer and fractional number support
- Bases 2-36 using digits 0-9 and letters A-Z
- Configurable precision for fractional parts
- Interactive mode for continuous conversions
- Input validation and error handling
- Decimal reference output for non-decimal conversions

## Testing

```bash
cargo test
```
