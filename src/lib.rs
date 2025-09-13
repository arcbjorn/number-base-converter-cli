pub mod converter;
pub mod formatter;
pub mod parser;

pub use converter::{convert_fractional_part, convert_integer_part};
pub use formatter::format_result;
pub use parser::parse_number;