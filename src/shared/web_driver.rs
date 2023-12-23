use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::{FirefoxCapabilities, WebDriver, CapabilitiesHelper};
use thirtyfour::prelude::*;
use crate::Error;
use std::path::Path;
use std::fs;
use crate::BROWSER;
use crate::TODAY;

pub async fn create(website: String) -> Result<WebDriver, Error> {
    let download_dir = format!("../tmp/{}/{}/docs", *TODAY, website);
    let dir = Path::new(&download_dir);
    fs::create_dir_all(dir)?;

    let abs_dir = fs::canonicalize(dir)?;
    let abs_dir_string = abs_dir.into_os_string().into_string().unwrap();

    match BROWSER  {
      "FireFox" => {
        let driver = create_gecko_driver(abs_dir_string).await?;
        Ok(driver)
      },
      "Chrome" => {
        let driver = create_chrome_driver(abs_dir_string).await?;
        Ok(driver)
      },
       _ => {
         return Err(Error::Other(format!("No browser chosen")));
      }
    }
}

async fn create_gecko_driver(abs_dir_string: String) -> Result<WebDriver, Error> {
  // Set user agent via Firefox preferences.
  let mut prefs = FirefoxPreferences::new();
  prefs.set("browser.download.folderList", 2)?;
  prefs.set("browser.download.manager.showWhenStarting", false)?;
  prefs.set("browser.download.dir", abs_dir_string)?;
  prefs.set("browser.helperApps.neverAsk.saveToDisk", "application/octet-stream".to_string())?;

  let mut caps = FirefoxCapabilities::new();
  caps.set_preferences(prefs)?;
  caps.accept_insecure_certs(true)?;
  let driver = WebDriver::new("http://localhost:4444", caps).await?;
  Ok(driver)
}

async fn create_chrome_driver(abs_dir_string: String) -> Result<WebDriver, Error> {
  let mut caps = DesiredCapabilities::chrome();
      caps.insert_browser_option(
        "prefs",
        serde_json::json!({
            "profile.default_content_settings": {
                "images": 2
            },
            "profile.managed_default_content_settings": {
                "images": 2
            },
            "profile.default_content_settings.popups": 0,
            "download.default_directory": abs_dir_string
        }),
      )?;
      let driver = WebDriver::new("http://localhost:9515", caps).await?;
      Ok(driver)
}