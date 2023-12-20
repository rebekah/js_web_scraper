use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::{FirefoxCapabilities, WebDriver, CapabilitiesHelper};
use thirtyfour::error::WebDriverError;
use thirtyfour::prelude::*;
use crate::WEBSITE_ENV_VAR_KEY as env_key;
use std::env;

pub async fn create(browser: &str, todays_dir: String) -> Result<WebDriver, WebDriverError> {

    if browser == "FireFox" {
      // Set user agent via Firefox preferences.
      let mut prefs = FirefoxPreferences::new();
      prefs.set("browser.download.folderList", 2);
      prefs.set("browser.download.manager.showWhenStarting", false);

      let website = match env::var(env_key){
        Ok(w) => w,
        Err(e) => "MyFloridaMarketPlace".to_string()
      };
      
      let downloadPath: String = format!("/Users/rwaterbury/dev/rust/tmp/{}/{}/docs", todays_dir, website);
      prefs.set("browser.download.dir", downloadPath);
      prefs.set("browser.helperApps.neverAsk.saveToDisk", "application/octet-stream".to_string());
      //prefs.set("acceptInsecureCerts", true);
      //prefs.set("marionette", false);
      
      let mut caps = FirefoxCapabilities::new();
      //caps.accept_ssl_certs(true)?;
      //caps.add_firefox_option("profile", "0bqe9yxe.default-release.zip")?;
      caps.set_preferences(prefs)?;
      caps.accept_insecure_certs(true);
      let driver = WebDriver::new("http://localhost:4444", caps).await?;
      Ok(driver)
    } else if browser == "Chrome" {
      let caps = DesiredCapabilities::chrome();
      let driver = WebDriver::new("http://localhost:9515", caps).await?;
      Ok(driver)
    } else {
      let caps = DesiredCapabilities::chrome();
      //caps.insert_browser_options(
      //  "prefs",
      //  serde_json::json!({
      //      "profile.default_content_settings": {
      //          "images": 2
      //      },
      //      "profile.managed_default_content_settings": {
      //          "images": 2
      //      },
      //      "profile.default_content_settings.popups": 0,
      //      "download.default_directory": "/Users/rwaterbury/dev/rust/tmp/docs"
      //  }),
      //)?;
      let driver = WebDriver::new("http://localhost:9515", caps).await?;
      Ok(driver)
      //let error: String = "A web driver could not be created".to_string();
      //return Err(WebDriverError::CustomError(error));
    }
}