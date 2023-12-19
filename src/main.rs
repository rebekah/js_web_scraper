use tokio;
use std::str::FromStr;
use strum_macros::EnumString;
use std::env;

//use thirtyfour::error::WebDriverError;

mod my_florida_marketplace;
mod shared;

use shared::scrape::run;

#[derive(Debug, PartialEq, EnumString)]
enum Website {
  MyFloridaMarketplace
}



#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    //error formatting
    color_eyre::install()?;
    //get website to scrape from command line args
    let args: Vec<String> = env::args().collect();
    let mut website = Website::MyFloridaMarketplace;
    if args.len() > 1 {
      let websiteFromArg: &String = &args[1];
      //println!(format!("{}", websiteFromArg));
      //let website = Website::from_str(websiteFromArg).unwrap();
      //println!("{:?}", website);
    }
      
    //scrape relevant html and docs from the website
    run(website).await?;
    //} //else {
      //let error: String = "No website referenced in command line argument".to_string();
      //return Err(WebDriverError::CustomError(error));
    //}
    Ok(())
}
