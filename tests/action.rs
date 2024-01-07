//! Actions tests
use crate::common::sample_page_url;
//use serial_test::serial;
use thirtyfour::prelude::*;

mod common;

async fn actions_key(c: WebDriver, port: u16) -> Result<(), WebDriverError> {
    let sample_url = sample_page_url(port);
    c.goto(&sample_url).await?;

    // Test key down/up.
    let elem = c.find(By::Id("text-input")).await?;
    elem.send_keys("a").await?;
    assert_eq!(elem.prop("value").await?.unwrap(), "a");

    elem.click().await?;
    c.action_chain().key_down(Key::Backspace).key_up(Key::Backspace).perform().await?;
    let elem = c.find(By::Id("text-input")).await?;
    assert_eq!(elem.prop("value").await?.unwrap(), "");
    Ok(())
}

mod chrome {
    use super::*;

    #[test]
    fn actions_key_test() {
        local_tester!(actions_key, "chrome");
    }
}

