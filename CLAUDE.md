# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Test Commands

```bash
cargo build              # Debug build
cargo build --release    # Release build
cargo test               # Run all tests
cargo test <test_name>   # Run a specific test
cargo run -- <args>      # Run with arguments
```

## CLI Usage

```bash
coingecko-fees-calculator -n <coin_id> -w <amount> [-c <currency>] [-p <precision>] [-f <fee_rate>]
```

Arguments:
- `-n, --coin-name`: CoinGecko API coin ID (e.g., "bitcoin", "xsgd")
- `-w, --withdraw-amount`: Amount being withdrawn
- `-c, --currency`: Target currency (default: "sgd")
- `-p, --precision`: Decimal precision (auto: 2 for fiat, 8 for crypto)
- `-f, --fees`: Fee rate (default: 0.0006)

## Architecture

**`src/lib.rs`** - Core business logic (testable, pure functions):
- `FIAT_CURRENCIES`: List of supported fiat currency codes
- `is_fiat_currency()`: Determines if a currency is fiat (case-insensitive)
- `get_default_precision()`: Returns precision based on currency type (2 for fiat, 8 for crypto)
- `calculate_fees()`: Computes withdrawal fees

**`src/main.rs`** - CLI and API integration:
- `Cli`: Clap-derived argument parser
- `MyConfig`: Configuration loaded via confy from `coingecko-config/config.toml`
- `Resp`: API response struct with custom `Display` impl for formatted output
- Async HTTP calls to CoinGecko API using reqwest/tokio

## Configuration

Config file location: `%APPDATA%\coingecko-config\config.toml` (Windows)

Required fields:
- `api_key`: CoinGecko API key
- `coingecko_api_url`: API base URL (default: `https://api.coingecko.com/api/v3`)

## Key Design Decisions

- Display formatting uses `$` prefix only for fiat currencies, omitted for crypto
- Precision auto-detection: fiat currencies default to 2 decimals, crypto to 8
- All numeric calculations use `f64` for precision with small cryptocurrency values
