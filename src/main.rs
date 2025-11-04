use serde::Deserialize;
use reqwest;
use anyhow::Result;
use clap::Parser;

#[derive(Deserialize)]
struct Coin {
    id: String,
    symbol: String,
    name: String,
    description: Description,
}

#[derive(Deserialize)]
struct Description {
    en: String,
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    coin: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let url = format!("https://api.coingecko.com/api/v3/coins/{}", args.coin);
    let res = reqwest::blocking::get(url)?;
    let coin: Coin = res.json()?;





    println!("{} {} {} {}", coin.id,
        coin.symbol,
             coin.name,
             coin.description.en);
    Ok(())

}
