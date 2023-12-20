use thirtyfour::prelude::*;
use thirtyfour::WebDriver;
use std::time::Duration;
use std::thread;
use thirtyfour::error::WebDriverError;
use regex::Regex;
use super::search::*;
use super::process::*;
use crate::Website;

//solution to the sites pagination bug
pub async fn navigate_pages(driver: WebDriver, three_seconds: Duration, one_second: Duration, todays_dir: &str)-> Result<WebDriver, WebDriverError>{
    let mut driver = driver;
    //Chrome
    thread::sleep(three_seconds);
    //FireFox
    let num_summaries_per_page = driver.find(By::XPath(
      "//div[contains(@class, 'mat-select-trigger')]//span[contains(@class, 'ng-star-inserted')]"
    )).await?.text().await?;
    //Chrome
    //let num_summaries_per_page_string = driver.find(By::XPath(
    //  "//span[contains(@class, 'totalfound')]"
    //)).await?.text().await?;
    //println!("{}", num_summaries_per_page_string);
    //let end = num_summaries_per_page_string.chars().map(|c| c.len_utf8()).take(2).sum();
    //let num_summaries_per_page = &num_summaries_per_page_string[..end];
    println!("{}", num_summaries_per_page);
    let num_summaries_per_page_float: f32 = num_summaries_per_page.parse::<f32>().unwrap();
    let total_found_text= driver.find(By::XPath("//span[contains(@class, 'totalfound')]")).await?.text().await?;
    let re = Regex::new(r"([0-9]+).+").unwrap();
    let caps = re.captures(&total_found_text);
    let total_summaries: f32 = match caps {
      None => {
        println!("nothing here");
        0.0
      },
      Some(captured) => {
        captured[1].parse::<f32>().unwrap()
      },
    };
    println!("summaries per page: {}", num_summaries_per_page_float);
    println!("total summaries text: {}", total_found_text);
    println!("summaries: {}", total_summaries);
    let number_of_pages = (total_summaries/num_summaries_per_page_float).ceil() as i32;
    println!("total number of pages: {}", number_of_pages);
    let mut page = 0;
    while page < number_of_pages {
      driver = load_summaries(driver, three_seconds, one_second).await?;
      let mut clicks = 0;
      while clicks < page {
        let next_page_arrow = driver.find(By::XPath("//button[@aria-label='Next page']")).await?;
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
        }
        clicks += 1;
        thread::sleep(three_seconds);
      }
      driver = process_summaries(driver, page, three_seconds, one_second, todays_dir).await?;
      page += 1;
    }
  
    Ok(driver)
  }