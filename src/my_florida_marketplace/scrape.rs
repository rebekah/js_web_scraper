use thirtyfour::WebDriver;
use super::search::load_summaries;
use super::navigate::navigate_pages;
use crate::Error;

pub async fn run(driver: WebDriver,) -> Result<WebDriver, Error> {
    let driver = load_summaries(driver).await?;
    let driver: WebDriver = navigate_pages(driver).await?;

    Ok(driver)
}