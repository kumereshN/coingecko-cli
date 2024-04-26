use std::collections::{HashMap, HashSet};
use thousands::Separable;
use coingecko_cli::calculate_fees;
use std::fmt;
use std::fmt::{Debug};
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Context, Result};
use clap::{Parser};

#[derive(Parser)]
#[command(name = "coingecko-cli",version = "1.0", about = "CoinGecko API CLI tool", long_about = None)]
pub struct Cli {
    #[arg(short, long = "coin–name")]
    name: String,
    #[arg(short, long = "currency", default_value = "sgd")]
    currency: String,
    #[arg(short = 'a', long = "current-amount")]
    current_amount: f32,
    #[arg(short, long, default_value = "2")]
    precision: String,
    #[arg(short, long, default_value = "0.0006")]
    fees: Option<f32>
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
    n_currencies: Option<usize>,
    #[serde(flatten)]
    prices: HashMap<String, HashMap<String, f64>>,
    fees: Option<f32>
}

impl fmt::Display for Resp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = &self.n_currencies.unwrap();
        let fees = &self.fees.unwrap();
        for (coin, prices) in &self.prices {
            write!(f, "The current price of {}", coin)?;
            for (i, (currency, price)) in prices.iter().enumerate() {
                if i == *n {
                    writeln!(f, " in {} is ${} and the fees are ${:.2}.", currency, price.separate_with_commas(), fees)?
                } else {
                    write!(f, " in {} is ${} and the fees are ${:.2}", currency, price.separate_with_commas(), fees)?
                }
            }
        }
        Ok(())
    }
}

async fn get_all_currencies(cfg: &MyConfig) -> Result<Vec<String>> {
    let coin_args = format!("{}/simple/supported_vs_currencies?x_cg_demo_api_key={}",
                            &cfg.coingecko_api_url, &cfg.api_key);
    let response = reqwest::get(coin_args).await?;
    let obj = response.json::<Vec<String>>().await?;
    Ok(obj)
}

#[tokio::main]
async fn main() -> Result<()> {
    // The config file can be found in C:\Users\Kumeresh\AppData\Roaming\coingecko-config
    let cfg: MyConfig = confy::load("coingecko-config", "test1")?;
    let cli = Cli::parse();


    if cli.name.contains(','){
        return Err(anyhow!("Invalid character found: ,"))
    }

    let vec_currencies = get_all_currencies(&cfg).await.context("Unable to get currencies")?;
    let cli_currency = cli.currency
        .split(',')
        .collect::<HashSet<&str>>()
        .iter()
        .filter_map(|&c| {
            match vec_currencies.contains(&c.to_string()) {
                true => Some(c),
                false => None
            }
        })
        .collect::<Vec<&str>>()
        .join(",");
    let coin_args = format!("{}/simple/price?x_cg_demo_api_key={}&ids={}&vs_currencies={}&precision={}",
                            &cfg.coingecko_api_url, &cfg.api_key, &cli.name, &cli_currency, &cli.precision);
    let response = reqwest::get(coin_args).await.context("Unable to obtain a response")?;
    let mut obj: Resp = response.json().await.context("Unable to parse response object to json")?;
    let price_of_coin = *obj.prices
        .get(&cli.name)
        .with_context(|| format!("Name of the coin: {} is incorrect", &cli.name))?
        .get(&cli_currency)
        .context("Unable to obtain the currency of the coin")? as f32;
    let fees = calculate_fees(cli.fees.unwrap(), price_of_coin);
    obj.n_currencies = Some(cli_currency.chars().filter(|&c| c == ',').count());
    obj.fees = Some(fees);

    println!("{}", obj);
    Ok(())
}
