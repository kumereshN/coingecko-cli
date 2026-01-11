/// List of supported fiat currencies
pub const FIAT_CURRENCIES: &[&str] = &[
    "usd", "eur", "gbp", "jpy", "aud", "cad", "chf", "cny", "hkd", "nzd", "sgd",
    "krw", "inr", "rub", "brl", "zar", "mxn", "idr", "try", "sar", "aed", "pln",
    "thb", "twd", "myr", "php", "vnd", "pkr", "bdt", "ngn", "uah", "ars", "clp",
    "cop", "pen", "czk", "dkk", "huf", "ils", "nok", "sek"
];

/// Check if a currency is a fiat currency (case-insensitive)
pub fn is_fiat_currency(currency: &str) -> bool {
    FIAT_CURRENCIES.contains(&currency.to_lowercase().as_str())
}

/// Get the default precision based on currency type
/// Returns 2 for fiat currencies, 8 for crypto currencies
/// If user_precision is provided, it takes precedence
pub fn get_default_precision(currency: &str, user_precision: Option<u8>) -> u8 {
    user_precision.unwrap_or_else(|| {
        if is_fiat_currency(currency) { 2 } else { 8 }
    })
}

/// Calculate withdrawal fees
pub fn calculate_fees(fees: f64, current_holdings: f64) -> f64 {
    current_holdings * fees
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== calculate_fees tests ====================

    #[test]
    fn test_calculate_fees_basic() {
        let result = calculate_fees(0.0006, 100.0);
        assert!((result - 0.06).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_fees_zero_fees() {
        let result = calculate_fees(0.0, 100.0);
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_fees_zero_holdings() {
        let result = calculate_fees(0.0006, 0.0);
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_fees_small_values() {
        // Simulating BTC-like small values
        let result = calculate_fees(0.0006, 0.00001);
        assert!((result - 0.000000006).abs() < 1e-15);
    }

    #[test]
    fn test_calculate_fees_large_values() {
        let result = calculate_fees(0.001, 1_000_000.0);
        assert!((result - 1000.0).abs() < f64::EPSILON);
    }

    // ==================== is_fiat_currency tests ====================

    #[test]
    fn test_is_fiat_currency_valid_lowercase() {
        assert!(is_fiat_currency("usd"));
        assert!(is_fiat_currency("sgd"));
        assert!(is_fiat_currency("eur"));
        assert!(is_fiat_currency("gbp"));
    }

    #[test]
    fn test_is_fiat_currency_valid_uppercase() {
        assert!(is_fiat_currency("USD"));
        assert!(is_fiat_currency("SGD"));
        assert!(is_fiat_currency("EUR"));
    }

    #[test]
    fn test_is_fiat_currency_valid_mixed_case() {
        assert!(is_fiat_currency("Usd"));
        assert!(is_fiat_currency("sGd"));
        assert!(is_fiat_currency("EuR"));
    }

    #[test]
    fn test_is_fiat_currency_crypto() {
        assert!(!is_fiat_currency("btc"));
        assert!(!is_fiat_currency("eth"));
        assert!(!is_fiat_currency("xrp"));
        assert!(!is_fiat_currency("BTC"));
    }

    #[test]
    fn test_is_fiat_currency_empty_string() {
        assert!(!is_fiat_currency(""));
    }

    #[test]
    fn test_is_fiat_currency_invalid() {
        assert!(!is_fiat_currency("xyz"));
        assert!(!is_fiat_currency("fake"));
        assert!(!is_fiat_currency("usdt")); // stablecoin, not fiat
    }

    // ==================== get_default_precision tests ====================

    #[test]
    fn test_default_precision_fiat_no_override() {
        assert_eq!(get_default_precision("sgd", None), 2);
        assert_eq!(get_default_precision("usd", None), 2);
        assert_eq!(get_default_precision("EUR", None), 2);
    }

    #[test]
    fn test_default_precision_crypto_no_override() {
        assert_eq!(get_default_precision("btc", None), 8);
        assert_eq!(get_default_precision("eth", None), 8);
        assert_eq!(get_default_precision("BTC", None), 8);
    }

    #[test]
    fn test_default_precision_fiat_with_override() {
        assert_eq!(get_default_precision("sgd", Some(4)), 4);
        assert_eq!(get_default_precision("usd", Some(6)), 6);
    }

    #[test]
    fn test_default_precision_crypto_with_override() {
        assert_eq!(get_default_precision("btc", Some(4)), 4);
        assert_eq!(get_default_precision("eth", Some(10)), 10);
    }

    #[test]
    fn test_default_precision_override_zero() {
        // User can set precision to 0 if they want
        assert_eq!(get_default_precision("btc", Some(0)), 0);
        assert_eq!(get_default_precision("sgd", Some(0)), 0);
    }
}
