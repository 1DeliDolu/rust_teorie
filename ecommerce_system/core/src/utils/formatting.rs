//! Formatting utilities

use rust_decimal::Decimal;

/// Format currency amount
pub fn format_currency(amount: Decimal, currency: &str) -> String {
    match currency.to_uppercase().as_str() {
        "USD" => format!("${:.2}", amount),
        "EUR" => format!("€{:.2}", amount),
        "GBP" => format!("£{:.2}", amount),
        "JPY" => format!("¥{:.0}", amount), // Yen typically has no decimal places
        _ => format!("{:.2} {}", amount, currency),
    }
}

/// Format product name for display (capitalize words)
pub fn format_product_name(name: &str) -> String {
    name.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Format file size in human readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Pluralize word based on count
pub fn pluralize(word: &str, count: usize) -> String {
    if count == 1 {
        format!("{} {}", count, word)
    } else {
        format!("{} {}s", count, word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(dec!(29.99), "USD"), "$29.99");
        assert_eq!(format_currency(dec!(50.00), "EUR"), "€50.00");
        assert_eq!(format_currency(dec!(1000), "JPY"), "¥1000");
    }

    #[test]
    fn test_format_product_name() {
        assert_eq!(format_product_name("hello world"), "Hello World");
        assert_eq!(format_product_name("LAPTOP computer"), "Laptop Computer");
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
    }

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize("item", 1), "1 item");
        assert_eq!(pluralize("item", 5), "5 items");
        assert_eq!(pluralize("product", 0), "0 products");
    }
}