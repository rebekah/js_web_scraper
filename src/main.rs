use tokio;
use std::str::FromStr;
use strum_macros::EnumString;
use std::env;

mod my_florida_marketplace;
mod shared;

use shared::scrape::run;

#[derive(Debug, PartialEq, EnumString)]
enum Website {
  MyFloridaMarketplace
}

pub const WEBSITE_ENV_VAR_KEY: &str = "WEBSITE";

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    //error formatting
    color_eyre::install()?;
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
