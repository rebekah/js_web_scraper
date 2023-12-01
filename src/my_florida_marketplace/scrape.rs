use thirtyfour::WebDriver;
use thirtyfour::error::WebDriverError;
use std::time::Duration;

use super::search::*;
use super::navigate::*;

pub async fn run(dir: &str, driver: WebDriver, three_seconds: Duration, one_second: Duration) -> Result<WebDriver, WebDriverError> {
    let driver = load_summaries(driver, three_seconds, one_second).await?;
    let driver: WebDriver = navigate_pages(driver, three_seconds, one_second, dir).await?;
    //println!("{}", url.as_str());
    Ok(driver)
}