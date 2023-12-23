use thirtyfour::prelude::*;
use thirtyfour::WebDriver;
use std::thread;
use regex::Regex;
use super::process::*;
use crate::Error;
use crate::THREE_SECONDS;
use crate::BROWSER;

//solution to the sites pagination bug
pub async fn navigate_pages(driver: WebDriver,)-> Result<WebDriver, Error>{
    thread::sleep(THREE_SECONDS);

    let summaries_per_page = get_summaries_per_page(&driver).await?;
    let total_summaries = get_total_summaries(&driver).await?;
    let number_of_pages = (total_summaries/summaries_per_page).ceil() as i32;
    let driver = traverse_pages(driver, number_of_pages).await?;
  
    Ok(driver)
  }

  async fn get_summaries_per_page(driver: &WebDriver) -> Result<f32, Error> {
    let num_summaries_per_page: Result<String, Error> = match BROWSER {
      "FireFox" => {
        let num_summaries_per_page = driver.find(By::XPath(
          "//div[contains(@class, 'mat-select-trigger')]//span[contains(@class, 'ng-star-inserted')]"
        )).await?.text().await?;
        Ok(num_summaries_per_page)
      },
      "Chrome" => {
        let num_summaries_per_page_string = driver.find(By::XPath(
          "//span[contains(@class, 'totalfound')]"
        )).await?.text().await?;
        let end = num_summaries_per_page_string.chars().map(|c| c.len_utf8()).take(2).sum();
        let num_summaries_per_page = num_summaries_per_page_string[..end].to_string();
        Ok(num_summaries_per_page)
      },
      _ => {
        return Err(Error::Other(format!("unexpected BROWSER value")));
      }
    };

    let num_summaries_per_page_float: f32 = num_summaries_per_page?.parse::<f32>().unwrap();
    Ok(num_summaries_per_page_float)
  }

  async fn get_total_summaries(driver: &WebDriver) -> Result<f32, Error> {
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
    Ok(total_summaries)
  }

  async fn traverse_pages(driver: WebDriver, number_of_pages: i32) -> Result<WebDriver, Error> {
    let mut driver = driver;

    let mut page = 0;
    while page < number_of_pages {
      driver = super::search::load_summaries(driver).await?;
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
        thread::sleep(THREE_SECONDS);
      }
      driver = process_summaries(driver).await?;
      page += 1;
    }
    
    Ok(driver)
  }