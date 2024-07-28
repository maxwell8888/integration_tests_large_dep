use chromiumoxide::{Browser, BrowserConfig};

pub async fn launch_browser() -> Result<(), Box<dyn std::error::Error>> {
    let (_browser, _handler) = Browser::launch(BrowserConfig::builder().build()?).await?;
    Ok(())
}
