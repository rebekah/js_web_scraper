use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::{FirefoxCapabilities, WebDriver};
use thirtyfour::error::WebDriverError;
use webdriver::error::ErrorStatus::*;
use std::borrow::Cow;

pub async fn create(browser: &str) -> Result<WebDriver, WebDriverError> {
    if browser == "Firefox" {
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
    } else {
      let error: String = "A web driver could not be created".to_string();
      return Err(WebDriverError::CustomError(error));
    }
}