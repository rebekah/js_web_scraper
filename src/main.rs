use tokio;
use strum_macros::EnumString;
use std::env;
use thiserror::Error;
use shared::scrape::run;
use thirtyfour::error::WebDriverError;
use std::env::VarError;
use std::io::Error as IoError;
use std::time::Duration;
use chrono::prelude::*;
use once_cell::sync::Lazy;


mod my_florida_marketplace;
mod shared;

#[derive(Debug, PartialEq, EnumString)]
enum Website {
  MyFloridaMarketplace
}

#[derive(Debug, Error)]
enum Error {
    #[error("web driver error: {0}")]
    WebDriver(#[from] WebDriverError),
    #[error("environment variable error: {0}")]
    EnvVar(#[from] VarError),
    #[error("input/output error: {0}")]
    IO(#[from] IoError),
    #[error("strum ParseError: {0}")]
    StrumParseError(#[from] strum::ParseError),
    #[error("error: {}", .0)]
    Other(String),
}

pub const WEBSITE_ENV_VAR_KEY: &str = "WEBSITE";
pub const BROWSER: &str = "Chrome";
pub const THREE_SECONDS: Duration = Duration::new(3, 0);
pub const ONE_SECOND: Duration = Duration::new(1, 0);

static UTC_NOW: Lazy<DateTime<Utc>> = Lazy::new(||Utc::now());
pub static TODAY: Lazy<String> = Lazy::new(||format!("{}-{}-{}",UTC_NOW.year(), UTC_NOW.month(), UTC_NOW.day()));

//example run command with command line arguments: cargo run MyFloridaMarketplace
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
      let website_from_arg: &String = &args[1];
      env::set_var(WEBSITE_ENV_VAR_KEY, website_from_arg.to_string());
    } else {
      return Err(
        Error::Other(format!("A Website enum value should be sent in as a command line argument."))
      );
    }

    run().await?;

    Ok(())
}
