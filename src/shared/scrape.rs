use crate::Error;
use crate::Website;
use crate::Website::*;
use thirtyfour::WebDriver;

pub async fn run(driver: WebDriver, website: Website)  -> Result<WebDriver, Error> {
    let mut driver = driver;

    //This match could become very long - possibly thousands
    match website {
      MyFloridaMarketplace => {
        driver = crate::my_florida_marketplace::scrape::run(driver).await?;
      }
    }
  
    Ok(driver)
}