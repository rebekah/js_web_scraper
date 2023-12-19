use thirtyfour::prelude::*;
use thirtyfour::WebDriver;
use std::time::Duration;
use std::thread;
use std::fs;
use thirtyfour::error::WebDriverError;

pub async fn process_summaries(driver: WebDriver, page: i32, three_seconds: Duration, one_second: Duration, todays_dir: &str) -> Result<WebDriver, WebDriverError> {
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
      thread::sleep(one_second);
      let proposal_title = driver.find(By::XPath("//h1[@class='mat-headline']")).await?.text().await?;
      print!("clicked into summary {}: {}\n", i.to_string(), proposal_title.to_string());
  
      //check to see if we've scraped this url before from history/this_website_domain_name(after the // and before the first slash)
  
      //create the directory for holding info related to this summary
      //add a file with meta-data - url, search terms, publish date, start date, end date in json format or yaml not sure
      //create a dir called docs inside of the proposal specific directory
  
      let html = driver.source().await?;
      let file_path = format!("/Users/rwaterbury/dev/rust/tmp/html/page_{}_proposal_{}", page.to_string(), i.to_string());
      //fs::write(file_path, html).expect("Unable to write file");
  
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
  
        //let handle = driver.window().await?;
        //downloads[i].click().await?;
        //thread::sleep(three_seconds);
        //driver.switch_to_window(handle).await?;
        thread::sleep(one_second);
      }
      driver.back().await?;
      thread::sleep(one_second);
    }
    Ok(driver)
  }