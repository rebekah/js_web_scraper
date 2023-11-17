use thirtyfour::prelude::*;
use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::{FirefoxCapabilities, WebDriver};
use tokio;
use std::time::Duration;
use std::thread;
use std::fs;
use thirtyfour::error::WebDriverError;
use async_recursion::async_recursion;

use thirtyfour::{
    components::{Component, ElementResolver},
    prelude::*,
    resolve,
    stringmatch::StringMatchable,
    support::sleep,
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    //error formatting
    color_eyre::install()?;

    //for sleeps - necessary when navigating the web
    let three_seconds = Duration::new(3, 0);
    let one_second = Duration::new(1, 0);

    //navigating the website to relevant RPS and scraping the html and docs
    let driver = create_web_driver().await?;
    let driver = navigate_to_summaries(driver, three_seconds).await?;
    let url = driver.current_url().await?;
    //println!("{}", url.as_str());
    let driver = hack_the_pagination_bug(driver, one_second, three_seconds, 1).await?;

    //it's necessary to manually quit the driver
    driver.quit().await?;

    Ok(())
}

async fn create_web_driver() -> Result<(WebDriver), WebDriverError> {
    // Set user agent via Firefox preferences.
    let mut prefs = FirefoxPreferences::new();
    prefs.set("browser.download.folderList", 2);
    prefs.set("browser.download.manager.showWhenStarting", false);
    prefs.set("browser.download.dir", "/Users/rwaterbury/dev/rust/tmp/docs".to_string());
    prefs.set("browser.helperApps.neverAsk.saveToDisk", "application/octet-stream".to_string());

    let mut caps = FirefoxCapabilities::new();
    caps.set_preferences(prefs)?;
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    Ok(driver)
}

async fn navigate_to_summaries(driver: WebDriver, three_seconds: Duration) -> Result<(WebDriver), WebDriverError> {
    let driver = go_to_advertisements(driver).await?;
    thread::sleep(three_seconds);
    let driver = choose_ad_type(driver).await?;
    let driver = choose_ad_status(driver).await?;
    let driver = click_search_button(driver).await?;
    thread::sleep(three_seconds);
    Ok(driver)
}

async fn go_to_advertisements(driver: WebDriver) -> Result<(WebDriver), WebDriverError>{
    driver.goto("https://vendor.myfloridamarketplace.com/search/bids").await?;
    Ok(driver)
}

async fn choose_ad_type(driver: WebDriver) -> Result<(WebDriver), WebDriverError> {
  let ad_type = driver.find(By::XPath("//span[contains(@class,'mat-content')]/mat-panel-title[contains(text(),'Ad Type')]")).await?;
  ad_type.click().await?;
  let req_for_prop_checkbox = driver.find(By::XPath("//div[contains(text(),' Request for Proposals ')]/preceding-sibling::mat-pseudo-checkbox")).await?;
  req_for_prop_checkbox.click().await?;
  let req_for_info_checkbox = driver.find(By::XPath("//div[contains(text(),' Request for Information ')]/preceding-sibling::mat-pseudo-checkbox")).await?;
  req_for_info_checkbox.click().await?;
  Ok(driver)
}
async fn choose_ad_status(driver: WebDriver) -> Result<(WebDriver), WebDriverError> {
  let ad_status = driver.find(By::XPath("//span[contains(@class,'mat-content')]/mat-panel-title[contains(text(),'Ad Status')]")).await?;
  ad_status.click().await?;
  let status_preview_checkbox = driver.find(By::XPath("//div[contains(text(),' PREVIEW ')]/preceding-sibling::mat-pseudo-checkbox")).await?;
  status_preview_checkbox.click().await?;
  let status_open_checkbox = driver.find(By::XPath("//div[contains(text(),' OPEN ')]/preceding-sibling::mat-pseudo-checkbox")).await?;
  status_open_checkbox.click().await?;
  Ok(driver)
}

async fn click_search_button(driver: WebDriver) -> Result<(WebDriver), WebDriverError>{
  let search_button = driver.find(By::XPath("//button[@type='submit']")).await?;
  search_button.click().await?;
  Ok(driver)
}

#[derive(Debug, Clone, Component)]
pub struct SummaryComponent {
    base: WebElement, // This is the <tr> element
    #[by(xpath = ".//span[@class='mat-button-wrapper']")]
    link: ElementResolver<WebElement>, // This is the <span /> element
}

impl SummaryComponent {
    pub async fn click_link(&self) -> WebDriverResult<()> {
      let summary_link = self.link.resolve_present().await?;
      if summary_link.is_clickable().await? {
        summary_link.click().await?;
      } else {
        println!("summary_link is not clickable");
      }
      Ok(())
    }
}

#[derive(Debug, Clone, Component)]
pub struct SummarySectionComponent {
    base: WebElement, // This is the outer <table>
    #[by(xpath = ".//tr[contains(@class, 'cdk-row')]", allow_empty)]
    rows: ElementResolver<Vec<SummaryComponent>>, // ElementResolver works with Components too.
}

#[async_recursion]
async fn hack_the_pagination_bug(driver: WebDriver, one_second: Duration, three_seconds: Duration, page: u8) -> Result<(WebDriver), WebDriverError> {
  let next_page_arrow = driver.find(By::XPath("//button[@aria-label='Next page']")).await?;
  let elem = driver.query(By::XPath("//table[contains(@class,'cdk-table')]")).first().await?;
  let component = SummarySectionComponent::from(elem);
  if next_page_arrow.is_clickable().await? {
    let blocking_div_exists = driver.query(By::XPath("//div[contains(@class, 'virtual-agent-button')]")).nowait().exists().await?;
    if blocking_div_exists {
      driver.execute(
          "(document.getElementsByClassName('virtual-agent-button')[0]).remove();",
          vec![],
      )
      .await?;
    }
    next_page_arrow.click().await?;
    thread::sleep(three_seconds);
    let driver = hack_the_pagination_bug(driver, one_second, three_seconds, page+1).await?;
    let url = driver.current_url().await?;
    //println!("{}", url.as_str());
    let driver = process_summaries(driver, page, three_seconds, one_second, component).await?;
    Ok(driver)
  } else {
    let url = driver.current_url().await?;
    //println!("{}", url.as_str());
    let driver = process_summaries(driver, page, three_seconds, one_second, component).await?;
    Ok(driver)
  }
}

async fn process_summaries(driver: WebDriver, page: u8, three_seconds: Duration, one_second: Duration, component: SummarySectionComponent) -> Result<(WebDriver), WebDriverError> {
  //let request_summaries = driver.find_all(
  //  By::XPath("//table[contains(@class,'cdk-table')]//tr[contains(@class, 'cdk-row')]//span[@class='mat-button-wrapper']")
  //).await?;
  println!("about to grab summaries");
  let rows = component.rows.resolve_present().await?;
  let num_summaries = rows.len();
  println!("{}", num_summaries);
  //let num_summaries = request_summaries.len();
  //for checkbox in checkboxes {
  //    checkbox.tick().await?;
  //}
  
  let handle = driver.window().await?;

  for row in rows {
    let blocking_div_exists = driver.query(By::XPath("//div[contains(@class, 'cdk-overlay-backdrop-showing')]")).nowait().exists().await?;
    if blocking_div_exists {
      driver.execute(
          "(document.getElementsByClassName('cdk-overlay-backdrop-showing')[0]).remove();",
          vec![],
      )
      .await?;
    }
    resolve!(row.link).click().await?;
    thread::sleep(three_seconds);
    let url = driver.current_url().await?;
    println!("{}", url.as_str());
    let proposal_title = driver.find(By::XPath("//h1[@class='mat-headline']")).await?.text().await?;
    print!("clicked into summary: {}\n", proposal_title.to_string());
    //let html = driver.source().await?;
    //let file_path = format!("/Users/rwaterbury/dev/rust/tmp/html/page_{}_proposal_{}", page.to_string(), i.to_string());
    //fs::write(file_path, html).expect("Unable to write file");
    let blocking_div_exists = driver.query(By::XPath("//div[contains(@class, 'cdk-overlay-backdrop-showing')]")).nowait().exists().await?;
    if blocking_div_exists {
      driver.execute(
          "(document.getElementsByClassName('cdk-overlay-backdrop-showing')[0]).remove();",
          vec![],
      )
      .await?;
    }

    let downloads = driver.find_all(By::XPath("//a[@class='document-link']")).await?;
    for i in 0..downloads.len() {
      //let handle = driver.window().await?;
      //downloads[i].click().await?;
      //thread::sleep(three_seconds);
      //driver.switch_to_window(handle).await?;
      //thread::sleep(one_second);
    }
    driver.back().await?;
    thread::sleep(one_second);
  }
  Ok(driver)

  //for i in 0..num_summaries {
  //  let summary = driver.find_all(By::XPath("//table[contains(@class,'cdk-table')]//tr[contains(@class, 'cdk-row')]//span[@class='mat-button-wrapper']")).await?;
  //  let blocking_div_exists = driver.query(By::XPath("//div[contains(@class, 'cdk-overlay-backdrop-showing')]")).nowait().exists().await?;
  //  if blocking_div_exists {
  //    driver.execute(
  //        "(document.getElementsByClassName('cdk-overlay-backdrop-showing')[0]).remove();",
  //        vec![],
  //    )
  //    .await?;
  //  }
  //  summary[i].click().await?;
  //  thread::sleep(one_second);
  //  let proposal_title = driver.find(By::XPath("//h1[@class='mat-headline']")).await?.text().await?;
  //  print!("clicked into summary {}: {}\n", i.to_string(), proposal_title.to_string());
  //  //let html = driver.source().await?;
  //  //let file_path = format!("/Users/rwaterbury/dev/rust/tmp/html/page_{}_proposal_{}", page.to_string(), i.to_string());
  //  //fs::write(file_path, html).expect("Unable to write file");
  //  let blocking_div_exists = driver.query(By::XPath("//div[contains(@class, 'cdk-overlay-backdrop-showing')]")).nowait().exists().await?;
  //  if blocking_div_exists {
  //    driver.execute(
  //        "(document.getElementsByClassName('cdk-overlay-backdrop-showing')[0]).remove();",
  //        vec![],
  //    )
  //    .await?;
  //  }
//
  //  let downloads = driver.find_all(By::XPath("//a[@class='document-link']")).await?;
  //  for i in 0..downloads.len() {
  //    //let handle = driver.window().await?;
  //    //downloads[i].click().await?;
  //    //thread::sleep(three_seconds);
  //    //driver.switch_to_window(handle).await?;
  //    //thread::sleep(one_second);
  //  }
  //  driver.back().await?;
  //  thread::sleep(one_second);
  //}
  //Ok(driver)
}
