use std::collections::HashMap;
use thousands::Separable;
use coingecko_fees_calculator::calculate_fees;
use std::fmt;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Context, Result};
use clap::{Parser};

#[derive(Parser)]
#[command(name = "coingecko-fees-calculator",version = "1.0", about = "CoinGecko API CLI tool for calculating fees", long_about = None)]
pub struct Cli {
    #[arg(short, long = "coin–name", help = "Name of the cryptocurrency", long_help = "Provide the api id of the cryptocurrency from coin gecko")]
    name: String,
    #[arg(short, long = "currency", help = "Target fiat currency", long_help = "Converts the cryptocurrency into the supported target fiat currencies", default_value = "sgd")]
    currency: String,
    #[arg(short = 'w', long = "withdraw-amount", long_help = "Cryptocurrency amount that is being withdrawn")]
    withdraw_amount: f32,
    #[arg(short, long, default_value = "2", long_help = "Decimal precision of the target cryptocurrency to fiat")]
    precision: u8,
    #[arg(short, long, default_value = "0.0006", long_help = "Cryptocurrency fees that needs to be paid for withdrawing")]
    fees: Option<f32>,
}

#[derive(Serialize, Deserialize)]
struct MyConfig {
    version: u8,
    coingecko_api_url: String,
    api_key: String,
}

impl Default for MyConfig {
    fn default() -> Self {
        Self {
            version: 0,
            coingecko_api_url: String::from("https://api.coingecko.com/api/v3"),
            api_key: String::from("")
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Resp {
    #[serde(flatten)]
    prices: HashMap<String, HashMap<String, f32>>,
    fees: Option<f32>,
    current_amount: Option<f32>
}

impl fmt::Display for Resp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fees = self.fees.context("Unable to obtain the fees").unwrap_or(0.0);
        for (coin, prices) in &self.prices {
            write!(f, "The current price of {}", coin)?;
            for (currency, price) in prices.iter() {
                let current_amount_in_fiat = self.current_amount.unwrap() * price;
                let fees_pct_over_withdraw_amount = (fees / current_amount_in_fiat) * 100_f32;
                let rounded_2_places_current_amount_in_fiat = format!("{:.2}",current_amount_in_fiat)
                    .parse::<f32>()
                    .context("Unable to convert current amount in fiat to 2 decimal places")
                    .unwrap()
                    .separate_with_commas();
                writeln!(f, " in {}: ${}\nWithdrawal amount: ${}\nWithdrawal fees: ${:.2}\nPercent of withdrawal fees over withdrawal amount: {:.2}%",
                         currency, price.separate_with_commas(), rounded_2_places_current_amount_in_fiat, fees, fees_pct_over_withdraw_amount)?
            }
        }
        Ok(())
    }
}

async fn get_currencies(cfg: &MyConfig) -> Result<Vec<String>> {
    let coin_args = format!("{}/simple/supported_vs_currencies?x_cg_demo_api_key={}",
                            &cfg.coingecko_api_url, &cfg.api_key);
    let response = reqwest::get(coin_args).await?;
    let obj = response.json::<Vec<String>>().await?;
    Ok(obj)
}

#[tokio::main]
async fn main() -> Result<()> {
    // The config file can be found in C:\Users\your_user_name\AppData\Roaming\coingecko-config
    let cfg: MyConfig = confy::load("coingecko-config", "test1")?;
    let cli = Cli::parse();


    if cli.name.contains(',') || cli.currency.contains(','){
        return Err(anyhow!("Invalid character: ',' found in either coin name or coin currency argument"))
    }

    let vec_currencies = get_currencies(&cfg).await.context("Unable to get currencies")?;
    if !vec_currencies.contains(&cli.currency) {
        return Err(anyhow!("{} is an invalid currency", cli.currency))
    }
    
    let coin_args = format!("{}/simple/price?x_cg_demo_api_key={}&ids={}&vs_currencies={}&precision={}",
                            &cfg.coingecko_api_url, &cfg.api_key, &cli.name, &cli.currency, &cli.precision);
    let response = reqwest::get(coin_args).await.context("Unable to obtain a response")?;
    let mut obj: Resp = response.json().await.context("Unable to parse response object to json")?;
    let price_of_coin = *obj.prices
        .get(&cli.name)
        .with_context(|| format!("Name of the coin/token: {} is incorrect or unsupported", &cli.name))?
        .get(&cli.currency)
        .context("Unable to obtain the currency of the coin")?;
    let fees = calculate_fees(cli.fees.unwrap(), price_of_coin);

    obj.fees = Some(fees);
    obj.current_amount = Some(cli.withdraw_amount);

    println!("{}", obj);
    Ok(())
}
