use tokio;
use std::str::FromStr;
use strum_macros::EnumString;
use std::env;
use thiserror::Error;

mod my_florida_marketplace;
mod shared;

use shared::scrape::run;
use thirtyfour::error::WebDriverError;
use std::env::VarError;
use std::io::Error as IoError;


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
    #[error("error: {}", .0)]
    Catchall(String),
}

pub const WEBSITE_ENV_VAR_KEY: &str = "WEBSITE";

#[tokio::main]
async fn main() -> Result<(), Error> {
    //get website to scrape from command line args
    let args: Vec<String> = env::args().collect();
    if(args.len() > 1) {
      let websiteFromArg: &String = &args[1];
      //println!(format!("{}", websiteFromArg));
      //let website = Website::from_str(websiteFromArg).unwrap();
      //println!("{:?}", website);
    } else {
      let website: String = format!("{:?}",Website::MyFloridaMarketplace);
      env::set_var(WEBSITE_ENV_VAR_KEY, website);
    }
      
    //scrape relevant html and docs from the website
    run().await?;
    //} //else {
      //let err&r: String = "No website referenced in command line argument".to_string();
      //return Err(WebDriverError::CustomError(error));
    //}
    Ok(())
}
