use thirtyfour::common::capabilities::firefox::FirefoxPreferences;
use thirtyfour::{FirefoxCapabilities, WebDriver, CapabilitiesHelper};
use thirtyfour::prelude::*;
use crate::WEBSITE_ENV_VAR_KEY as env_key;
use std::env;
use crate::Error;
use std::path::Path;
use std::fs;

pub async fn create(browser: &str, todays_dir: String) -> Result<WebDriver, Error> {

    let website = env::var(env_key)?;
    let download_dir = format!("../tmp/{}/{}/docs", todays_dir, website);
    let dir = Path::new(&download_dir);
    fs::create_dir_all(dir)?;
    let abs_dir = fs::canonicalize(dir)?;
    let abs_dir_string = abs_dir.into_os_string().into_string().unwrap();
    println!("{}", abs_dir_string);

    if browser == "FireFox" {
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
    } else if browser == "Chrome" {
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
    } else {
      return Err(Error::Catchall(format!("No browser chosen")));
    }
}