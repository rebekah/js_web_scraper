use thirtyfour::prelude::*;
use thirtyfour::WebDriver;
use std::thread;
use std::fs;
use crate::{ONE_SECOND,THREE_SECONDS,TODAY};
use crate::WEBSITE_ENV_VAR_KEY as env_key;
use std::env;
use crate::Error;
use std::path::Path;

pub async fn process_summaries(driver: WebDriver) -> Result<WebDriver, Error> {
    let request_summaries = driver.find_all(
      By::XPath("//table[contains(@class,'cdk-table')]//tr[contains(@class, 'cdk-row')]//span[@class='mat-button-wrapper']")
    ).await?;
    println!("about to grab summaries");
  
    let num_summaries = request_summaries.len();
  
    for i in 0..num_summaries {
      let summary = driver.find_all(By::XPath("//table[contains(@class,'cdk-table')]//tr[contains(@class, 'cdk-row')]//span[@class='mat-button-wrapper']")).await?;
      let blocking_div_exists = driver.query(By::XPath("//div[contains(@class, 'cdk-overlay-backdrop-showing')]")).nowait().exists().await?;
      if blocking_div_exists {
        driver.execute(
            "(document.getElementsByClassName('cdk-overlay-backdrop-showing')[0]).remove();",
            vec![],
        )
        .await?;
      }
      summary[i].click().await?;
      thread::sleep(ONE_SECOND);
      let summary_url = driver.current_url().await?.to_string();
      println!("{}", summary_url);
      let proposal_title = driver.find(By::XPath("//h1[@class='mat-headline']")).await?.text().await?;
      print!("clicked into summary {}: {}\n", i.to_string(), proposal_title.to_string());
  
      //check to see if we've scraped this url before from history/this_website_domain_name(after the // and before the first slash)
  
      //create the directory for holding info related to this summary
      //add a file with meta-data - url, search terms, publish date, start date, end date in json format or yaml not sure
      //create a dir called docs inside of the proposal specific directory
  
      let html = driver.source().await?;

      let website = match env::var(env_key){
        Ok(w) => w,
        Err(_) => "MyFloridaMarketPlace".to_string()
      };
      
      
      let file_path = format!("../tmp/{}/{}/html/{}.html", TODAY.to_string(), website, i);
      let file = Path::new(&file_path);
      let file_dir = format!("../tmp/{}/{}/html", TODAY.to_string(), website);
      let dir = Path::new(&file_dir);
      fs::create_dir_all(dir)?;
      fs::write(file, html).expect("Unable to write file");
  
      let downloads = driver.find_all(By::XPath("//a[@class='document-link']")).await?;
      for i in 0..downloads.len() {
        let blocking_div_exists = driver.query(By::XPath("//div[contains(@class, 'cdk-overlay-backdrop-showing')]")).nowait().exists().await?;
        if blocking_div_exists {
          let result = driver.execute(
              "(document.getElementsByClassName('cdk-overlay-backdrop-showing')[0]).remove();",
              vec![],
          )
          .await;
          match result {
            Ok(_) => {},
            Err(_) => {
              println!("There was an issue removing the overlay element.")
            }
          }
        }
  
        let handle = driver.window().await?;
        downloads[i].click().await?;
        thread::sleep(THREE_SECONDS);
        driver.switch_to_window(handle).await?;
        thread::sleep(ONE_SECOND);
      }
      driver.back().await?;
      thread::sleep(ONE_SECOND);
    }
    Ok(driver)
  }