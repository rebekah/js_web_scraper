use thirtyfour::WebDriver;
use std::time::Duration;
use super::search::*;
use super::navigate::*;
use crate::Error;

pub async fn run(dir: &str, driver: WebDriver, three_seconds: Duration, one_second: Duration) -> Result<WebDriver, Error> {
    let driver = load_summaries(driver, three_seconds, one_second).await?;
    let driver: WebDriver = navigate_pages(driver, three_seconds, one_second, dir).await?;

    Ok(driver)
}