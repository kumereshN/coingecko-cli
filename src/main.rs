use std::collections::HashMap;
use thousands::Separable;
use std::fmt;
use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Parser)]
#[command(name = "coingecko-cli",version = "1.0", about = "CoinGecko API CLI tool", long_about = None)]
struct Cli {
    #[arg(short, long = "coin–name")]
    name: String,
    #[arg(short, long = "currency")]
    currency: String,
    #[arg(short, long)]
    #[arg(default_value = "2")]
    precision: String
}

#[derive(Serialize, Deserialize)]
struct MyConfig {
    version: u8,
    coingecko_api_url: String,
    api_key: String,
}

#[derive(Deserialize, Debug)]
struct Resp {
    n_currencies: Option<usize>,
    #[serde(flatten)]
    prices: HashMap<String, HashMap<String, f64>>

}

impl fmt::Display for Resp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = &self.n_currencies.unwrap();
        for (coin, prices) in &self.prices {
            write!(f, "The price of {}", coin)?;
            for (i, (currency, price)) in prices.iter().enumerate() {
                if i == *n {
                    writeln!(f, " in {} is ${}.", currency, price.separate_with_commas())?
                } else {
                    write!(f, " in {} is ${},", currency, price.separate_with_commas())?
                }
            }
        }
        Ok(())
    }
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

#[derive(Debug, Serialize)]
pub enum CoinGeckoEndpoints {
    Simple(u32)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The config file can be found in C:\Users\Kumeresh\AppData\Roaming\coingecko-config
    let cfg: MyConfig = confy::load("coingecko-config", "test1")?;
    let cli = Cli::parse();

    let coin_args = format!("{}/simple/price?x_cg_demo_api_key={}&ids={}&vs_currencies={}&precision={}",
                            &cfg.coingecko_api_url, &cfg.api_key, &cli.name, &cli.currency, &cli.precision);
    let response = reqwest::get(coin_args).await?;
    let mut obj: Resp = response.json().await?;
    obj.n_currencies = Some(cli.currency.chars().filter(|&c| c == ',').count());

    println!("{}", obj);
    Ok(())
}
