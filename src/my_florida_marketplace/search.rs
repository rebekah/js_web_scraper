use thirtyfour::prelude::*;
use thirtyfour::WebDriver;
use std::thread;
use crate::Error;
use crate::{THREE_SECONDS, ONE_SECOND, BROWSER};

pub async fn load_summaries(driver: WebDriver) -> Result<WebDriver, Error> {
    let driver = go_to_advertisements(driver).await?;
    thread::sleep(THREE_SECONDS);
    let driver = choose_ad_type(driver).await?;
    let driver = choose_ad_status(driver).await?;
    let driver = click_search_button(driver).await?;
    thread::sleep(THREE_SECONDS);
    Ok(driver)
}

async fn go_to_advertisements(driver: WebDriver) -> Result<WebDriver, Error>{
    driver.goto("https://vendor.myfloridamarketplace.com/search/bids").await?;
    thread::sleep(ONE_SECOND);
    Ok(driver)
}

async fn choose_ad_type(driver: WebDriver) -> Result<WebDriver, Error> {
  thread::sleep(ONE_SECOND);
  let ad_type = driver.find(By::XPath("//span[contains(@class,'mat-content')]/mat-panel-title[contains(text(),'Ad Type')]")).await?;
  ad_type.click().await?;
  if BROWSER == "Chrome" {
    thread::sleep(ONE_SECOND);
  }
  let req_for_prop_checkbox = driver.find(By::XPath("//div[contains(text(),' Request for Proposals ')]/preceding-sibling::mat-pseudo-checkbox")).await?;
  req_for_prop_checkbox.click().await?;
  let req_for_info_checkbox = driver.find(By::XPath("//div[contains(text(),' Request for Information ')]/preceding-sibling::mat-pseudo-checkbox")).await?;
  req_for_info_checkbox.click().await?;
  Ok(driver)
}
async fn choose_ad_status(driver: WebDriver) -> Result<WebDriver, Error> {
  let ad_status = driver.find(By::XPath("//span[contains(@class,'mat-content')]/mat-panel-title[contains(text(),'Ad Status')]")).await?;
  ad_status.click().await?;
  if BROWSER == "Chrome" {
    thread::sleep(ONE_SECOND);
  }
  let status_preview_checkbox = driver.find(By::XPath("//div[contains(text(),' PREVIEW ')]/preceding-sibling::mat-pseudo-checkbox")).await?;
  status_preview_checkbox.click().await?;
  let status_open_checkbox = driver.find(By::XPath("//div[contains(text(),' OPEN ')]/preceding-sibling::mat-pseudo-checkbox")).await?;
  status_open_checkbox.click().await?;
  Ok(driver)
}

async fn click_search_button(driver: WebDriver) -> Result<WebDriver, Error>{
  let search_button = driver.find(By::XPath("//button[@type='submit']")).await?;
  search_button.click().await?;
  Ok(driver)
}