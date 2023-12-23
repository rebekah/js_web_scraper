use std::time::Duration;
use chrono::prelude::*;
use thirtyfour::WebDriver;
use crate::Error;
use crate::WEBSITE_ENV_VAR_KEY as env_key;
use std::env;
use crate::Website;
use crate::Website::*;
use std::str::FromStr;

pub async fn run() -> Result<(), Error> {
    //for sleeps - necessary when navigating the web
    const THREE_SECONDS: Duration = Duration::new(3, 0);
    const ONE_SECOND: Duration = Duration::new(1, 0);

    let utc: DateTime<Utc> = Utc::now();
    let todays_dir = format!("{}-{}-{}",utc.year(), utc.month(), utc.day());
    println!("{}", todays_dir);
    
    scrape_one_website(&todays_dir, THREE_SECONDS, ONE_SECOND).await?;
    
    Ok(())
}

//navigating a website to relevant RPS and scraping the html and docs
async fn scrape_one_website(
    todays_dir: &str,
    three_seconds: Duration,
    one_second: Duration
) -> Result<(), Error> {
    let website_str = env::var(env_key)?;
    let website = Website::from_str(&website_str)?;

    match website {
      MyFloridaMarketplace => {
        let mut driver: WebDriver = super::web_driver::create("Chrome",todays_dir.to_string(), website_str).await?;
        driver = crate::my_florida_marketplace::scrape::run(&todays_dir, driver, three_seconds, one_second).await?;
        
        //it's necessary to manually quit the driver
        driver.quit().await?;
      },
      _ => {
        return Err(Error::Other(format!("website not in Website Enum")));
      }
    };
  
    Ok(())
}