use playwright_rs::{install_browsers, Error, LaunchOptions, Playwright};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let playwright = Playwright::launch().await?;
    let launch_options = LaunchOptions::new().headless(false);
    let browser = match playwright
        .chromium()
        .launch_with_options(launch_options.clone())
        .await
    {
        Ok(browser) => browser,
        Err(Error::BrowserNotInstalled {
            browser_name: _,
            message: _,
            playwright_version: _,
        }) => {
            println!("Installing browser toolkit.");
            // installing into %USERPROFILE%\AppData\Local\ms-playwright (Win)
            //                  ~/Library/Caches/ms-playwright (macOS)
            install_browsers(Some(&["chromium"])).await?;
            playwright
                .chromium()
                .launch_with_options(launch_options)
                .await?
        }
        Err(e) => return Err(e)?,
    };

    let page = browser.new_page().await?;


    let _ = page.goto("https://lms.bsuir.by/login/auth.php", None).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    browser.close().await?;
    Ok(())
}
