use base_converter::{convert_fractional_part, convert_integer_part, format_result, parse_number};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(about = "Convert numbers between different base systems (supports fractional values)")]
struct Args {
    #[arg(short, long)]
    value: String,

    #[arg(short = 's', long)]
    from_base: u32,

    #[arg(short = 't', long)]
    to_base: u32,

    #[arg(short = 'p', long, default_value_t = 10)]
    precision: usize,
}

fn main() {
    let args = Args::parse();

    if args.from_base < 2 || args.from_base > 36 {
        eprintln!("Error: Source base must be between 2 and 36");
        std::process::exit(1);
    }
    
    if args.to_base < 2 || args.to_base > 36 {
        eprintln!("Error: Target base must be between 2 and 36");
        std::process::exit(1);
    }

    let (integer_part, fractional_part) = match parse_number(&args.value, args.from_base) {
        Ok(parts) => parts,
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            std::process::exit(1);
        }
    };

    let converted_integer = convert_integer_part(&integer_part, args.from_base, args.to_base);
    let converted_fractional = convert_fractional_part(
        &fractional_part,
        args.from_base,
        args.to_base,
        args.precision,
    );

    let result = format_result(&converted_integer, &converted_fractional);
    
    println!("Input: {} (base {})", args.value, args.from_base);
    println!("Result: {} (base {})", result, args.to_base);
    
    if args.from_base != 10 || args.to_base != 10 {
        let decimal_integer = convert_integer_part(&integer_part, args.from_base, 10);
        let decimal_fractional = convert_fractional_part(&fractional_part, args.from_base, 10, 10);
        let decimal_result = format_result(&decimal_integer, &decimal_fractional);
        println!("Decimal: {}", decimal_result);
    }
}
