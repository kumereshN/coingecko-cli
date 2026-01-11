# CoinGecko Fees Calculator

A command-line tool for calculating cryptocurrency withdrawal fees using real-time prices from the CoinGecko API.

## Features

- Fetches real-time cryptocurrency prices from CoinGecko
- Calculates withdrawal fees based on current market prices
- Supports both fiat currencies (USD, SGD, EUR, etc.) and crypto currencies (BTC, ETH, etc.)
- Auto-detects appropriate decimal precision (2 for fiat, 8 for crypto)
- Displays fee percentage relative to withdrawal amount

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/coingecko-fees-calculator`.

## Configuration

Create a config file at:
- Windows: `%APPDATA%\coingecko-config\config.toml`
- Linux/macOS: `~/.config/coingecko-config/config.toml`

```toml
version = 0
coingecko_api_url = "https://api.coingecko.com/api/v3"
api_key = "your-coingecko-api-key"
```

## Usage

```bash
coingecko-fees-calculator -n <coin_id> -w <amount> [OPTIONS]
```

### Arguments

| Flag | Long | Description | Default |
|------|------|-------------|---------|
| `-n` | `--coin-name` | CoinGecko coin ID (e.g., "bitcoin", "ethereum", "xsgd") | Required |
| `-w` | `--withdraw-amount` | Amount of cryptocurrency being withdrawn | Required |
| `-c` | `--currency` | Target currency for conversion | `sgd` |
| `-p` | `--precision` | Decimal precision for display | Auto (2 fiat, 8 crypto) |
| `-f` | `--fees` | Withdrawal fee rate | `0.0006` |

### Examples

```bash
# Calculate fees for withdrawing 1000 XSGD to SGD
coingecko-fees-calculator -n xsgd -w 1000 -c sgd

# Calculate fees for withdrawing 1000 XSGD to BTC
coingecko-fees-calculator -n xsgd -w 1000 -c btc

# Calculate fees with custom precision
coingecko-fees-calculator -n bitcoin -w 0.5 -c usd -p 4

# Calculate fees with custom fee rate
coingecko-fees-calculator -n ethereum -w 10 -c eur -f 0.001
```

### Sample Output

```
The current price of xsgd in sgd: $1.00
Withdrawal amount: $1,000
Withdrawal fees: $0.00
Percent of withdrawal fees over withdrawal amount: 0.06%
```

For crypto currencies, the `$` prefix is omitted:

```
The current price of xsgd in btc: 0.00000859
Withdrawal amount: 0.00859
Withdrawal fees: 0.00000001
Percent of withdrawal fees over withdrawal amount: 0.00%
```

## Running Tests

```bash
cargo test
```

## License

MIT
