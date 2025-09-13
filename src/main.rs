use base_converter::{convert_fractional_part, convert_integer_part, format_result, parse_number};
use clap::Parser;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(about = "Convert numbers between different base systems (supports fractional values)")]
struct Args {
    #[arg(short, long, conflicts_with = "interactive")]
    value: Option<String>,

    #[arg(short = 's', long, conflicts_with = "interactive")]
    from_base: Option<u32>,

    #[arg(short = 't', long, conflicts_with = "interactive")]
    to_base: Option<u32>,

    #[arg(short = 'p', long, default_value_t = 10)]
    precision: usize,

    #[arg(short, long, help = "Run in interactive mode")]
    interactive: bool,
}

fn convert_number(value: &str, from_base: u32, to_base: u32, precision: usize) {
    if from_base < 2 || from_base > 36 {
        eprintln!("Error: Source base must be between 2 and 36");
        return;
    }
    
    if to_base < 2 || to_base > 36 {
        eprintln!("Error: Target base must be between 2 and 36");
        return;
    }

    let (integer_part, fractional_part) = match parse_number(value, from_base) {
        Ok(parts) => parts,
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            return;
        }
    };

    let converted_integer = convert_integer_part(&integer_part, from_base, to_base);
    let converted_fractional = convert_fractional_part(
        &fractional_part,
        from_base,
        to_base,
        precision,
    );

    let result = format_result(&converted_integer, &converted_fractional);
    
    println!("Input: {} (base {})", value, from_base);
    println!("Result: {} (base {})", result, to_base);
    
    if from_base != 10 || to_base != 10 {
        let decimal_integer = convert_integer_part(&integer_part, from_base, 10);
        let decimal_fractional = convert_fractional_part(&fractional_part, from_base, 10, 10);
        let decimal_result = format_result(&decimal_integer, &decimal_fractional);
        println!("Decimal: {}", decimal_result);
    }
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_base(input: &str) -> Option<u32> {
    input.parse::<u32>().ok()
}

fn interactive_mode(precision: usize) {
    println!("Base Converter - Interactive Mode");
    println!("Type 'quit' or 'exit' to leave, 'help' for instructions\n");

    loop {
        let value = read_line("Enter number to convert (or command): ");
        
        if value.is_empty() {
            continue;
        }
        
        match value.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "help" => {
                println!("\nHow to use:");
                println!("1. Enter the number you want to convert");
                println!("2. Enter the source base (2-36)");
                println!("3. Enter the target base (2-36)");
                println!("Commands: 'quit'/'exit' to leave, 'help' for this message\n");
                continue;
            }
            _ => {}
        }

        let from_base_str = read_line("Enter source base (2-36): ");
        let from_base = match parse_base(&from_base_str) {
            Some(base) if base >= 2 && base <= 36 => base,
            _ => {
                eprintln!("Invalid source base. Please enter a number between 2 and 36.\n");
                continue;
            }
        };

        let to_base_str = read_line("Enter target base (2-36): ");
        let to_base = match parse_base(&to_base_str) {
            Some(base) if base >= 2 && base <= 36 => base,
            _ => {
                eprintln!("Invalid target base. Please enter a number between 2 and 36.\n");
                continue;
            }
        };

        println!();
        convert_number(&value, from_base, to_base, precision);
        println!();
    }
}

fn main() {
    let args = Args::parse();

    if args.interactive {
        interactive_mode(args.precision);
    } else {
        let value = args.value.unwrap_or_else(|| {
            eprintln!("Error: --value is required when not in interactive mode");
            std::process::exit(1);
        });
        
        let from_base = args.from_base.unwrap_or_else(|| {
            eprintln!("Error: --from-base is required when not in interactive mode");
            std::process::exit(1);
        });
        
        let to_base = args.to_base.unwrap_or_else(|| {
            eprintln!("Error: --to-base is required when not in interactive mode");
            std::process::exit(1);
        });

        convert_number(&value, from_base, to_base, args.precision);
    }
}
