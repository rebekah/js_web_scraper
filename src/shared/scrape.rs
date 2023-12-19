use std::time::Duration;
//use std::fs;
use chrono::prelude::*;
use thirtyfour::WebDriver;
use thirtyfour::error::WebDriverError;
use crate::Website::*;
use crate::Website;
use std::thread;

pub async fn run(website: Website) -> Result<(), WebDriverError> {
    //for sleeps - necessary when navigating the web
    const THREE_SECONDS: Duration = Duration::new(3, 0);
    const ONE_SECOND: Duration = Duration::new(1, 0);

    //create directory for today - this should be at the code that runs through websites, not in the code that scrapes one specific website - these process should be in seperagte modules in the run method or something like that
    const BASE_OUTPUT_DIR: &str = "/Users/rwaterbury/dev/rust/js_web_scraper/tmp";
    let utc: DateTime<Utc> = Utc::now();
    let todays_dir = format!("{}/{}-{}-{}",BASE_OUTPUT_DIR, utc.year(), utc.month(), utc.day());
    println!("{}", todays_dir);
    //fs::create_dir(&todays_dir)?;
    
    scrape_one_website(&todays_dir, website, THREE_SECONDS, ONE_SECOND).await?;
    
    Ok(())
}

//navigating a website to relevant RPS and scraping the html and docs
async fn scrape_one_website(
    todays_dir: &str,
    website: Website,
    three_seconds: Duration,
    one_second: Duration
) -> Result<(), WebDriverError> {

    let mut driver: WebDriver = super::web_driver::create("FireFox").await?;
    //match website {
    //  MyFloridaMarketplace => {
    //    //update with download dir, etc.
    //    //driver = super::web_driver::create("Firefox").await?;
    //    driver = crate::my_florida_marketplace::scrape::run(&todays_dir, driver, three_seconds, one_second).await?;
    //  }
    //  //let error = "the commandline arg is not in website list: [MYFLORIDAMARKETPLACE].";
    //  //return Err(WebDriverError::CustomError(error.to_string()));
    //}

    driver = crate::my_florida_marketplace::scrape::run(&todays_dir, driver, three_seconds, one_second).await?;

    //it's necessary to manually quit the driver
    driver.quit().await?;
      
    Ok(())
}