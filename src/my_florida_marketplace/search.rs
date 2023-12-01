use thirtyfour::prelude::*;
use thirtyfour::WebDriver;
use std::time::Duration;
use std::thread;
use thirtyfour::error::WebDriverError;

pub async fn load_summaries(driver: WebDriver, three_seconds: Duration, one_second: Duration) -> Result<WebDriver, WebDriverError> {
    let driver = go_to_advertisements(driver, one_second).await?;
    thread::sleep(three_seconds);
    let driver = choose_ad_type(driver).await?;
    let driver = choose_ad_status(driver).await?;
    let driver = click_search_button(driver).await?;
    thread::sleep(three_seconds);
    Ok(driver)
}

async fn go_to_advertisements(driver: WebDriver, one_second: Duration) -> Result<WebDriver, WebDriverError>{
    driver.goto("https://vendor.myfloridamarketplace.com/search/bids").await?;
    let advanced_button = driver.find(By::XPath("//button[@id='advancedButton']")).await?;
    //thread::sleep(one_second);
    //advanced_button.click().await?;
    //let exception_button = driver.find(By::XPath("//button[@id='exceptionDialogButton']")).await?;
    //exception_button.click();
    //thread::sleep(one_second);
    Ok(driver)
}

async fn choose_ad_type(driver: WebDriver) -> Result<WebDriver, WebDriverError> {
  let ad_type = driver.find(By::XPath("//span[contains(@class,'mat-content')]/mat-panel-title[contains(text(),'Ad Type')]")).await?;
  ad_type.click().await?;
  let req_for_prop_checkbox = driver.find(By::XPath("//div[contains(text(),' Request for Proposals ')]/preceding-sibling::mat-pseudo-checkbox")).await?;
  req_for_prop_checkbox.click().await?;
  let req_for_info_checkbox = driver.find(By::XPath("//div[contains(text(),' Request for Information ')]/preceding-sibling::mat-pseudo-checkbox")).await?;
  req_for_info_checkbox.click().await?;
  Ok(driver)
}
async fn choose_ad_status(driver: WebDriver) -> Result<WebDriver, WebDriverError> {
  let ad_status = driver.find(By::XPath("//span[contains(@class,'mat-content')]/mat-panel-title[contains(text(),'Ad Status')]")).await?;
  ad_status.click().await?;
  let status_preview_checkbox = driver.find(By::XPath("//div[contains(text(),' PREVIEW ')]/preceding-sibling::mat-pseudo-checkbox")).await?;
  status_preview_checkbox.click().await?;
  let status_open_checkbox = driver.find(By::XPath("//div[contains(text(),' OPEN ')]/preceding-sibling::mat-pseudo-checkbox")).await?;
  status_open_checkbox.click().await?;
  Ok(driver)
}

async fn click_search_button(driver: WebDriver) -> Result<WebDriver, WebDriverError>{
  let search_button = driver.find(By::XPath("//button[@type='submit']")).await?;
  search_button.click().await?;
  Ok(driver)
}