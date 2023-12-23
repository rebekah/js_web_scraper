use thirtyfour::WebDriver;
use crate::Error;
use crate::WEBSITE_ENV_VAR_KEY as env_key;
use std::env;
use crate::Website;
use crate::Website::*;
use std::str::FromStr;
use crate::{ONE_SECOND, THREE_SECONDS, TODAY};

pub async fn run()  -> Result<(), Error> {
    let website_str = env::var(env_key)?;
    let website = Website::from_str(&website_str)?;

    match website {
      MyFloridaMarketplace => {
        let mut driver: WebDriver = super::web_driver::create(website_str).await?;
        
        driver = crate::my_florida_marketplace::scrape::run(driver).await?;
        
        //it's necessary to manually quit the driver
        driver.quit().await?;
      },
      _ => {
        return Err(Error::Other(format!("website not in Website Enum")));
      }
    };
  
    Ok(())
}