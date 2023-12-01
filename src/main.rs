use tokio;
//use std::env;
//use thirtyfour::error::WebDriverError;

mod my_florida_marketplace;
mod shared;

use shared::scrape::run;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    //error formatting
    color_eyre::install()?;
    //get website to scrape from command line args
    //let args: Vec<String> = env::args().collect();
    //if args.len() > 1 {
      //let website = &args[1];
      //let website = "MYFLORIDAMARKETPLACE";
      
      //scrape relevant html and docs from the website
      run().await?;
    //} //else {
      //let error: String = "No website referenced in command line argument".to_string();
      //return Err(WebDriverError::CustomError(error));
    //}
    Ok(())
}
