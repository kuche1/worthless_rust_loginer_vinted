use thirtyfour::prelude::*; // cargo add thirtyfour
use tokio; // cargo add tokio --features rt-multi-thread
use std::io;
use std::io::prelude::*;
use std::time::Duration;
use std::thread;

// btn_class = web_ui__Button__button web_ui__Button__flat web_ui__Button__default web_ui__Button__primary web_ui__Button__inline web_ui__Button__truncated web_ui__Button__icon-left web_ui__Button__without-text

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press enter to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

#[tokio::main]
async fn main() {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await.expect("make sure that you have `chromedriver` installed an running on port `9515` using `chromedriver --port=9515`, as well as that you have the chrome binary");

    driver.goto("https://www.vinted.co.uk/").await.unwrap();

    thread::sleep(Duration::from_millis(400));

    let btn_close_where_do_you_live = driver.find(By::XPath(r"/html/body/div[4]/div/div/div/div[1]/div/div[3]/button")).await.unwrap();
    btn_close_where_do_you_live.click().await.unwrap();

    let btn_accept_all_cookies = driver.find(By::XPath(r#"//*[@id="onetrust-accept-btn-handler"]"#)).await.unwrap();
    btn_accept_all_cookies.click().await.unwrap();

    let btn_login = driver.find(By::XPath(r"/html/body/div[1]/div/div[1]/header/div/div/div[3]/div/a[1]")).await.unwrap();
    btn_login.click().await.unwrap();

    // thread::sleep(Duration::from_millis(300));

    // let btn_login_with_mail = driver.find(By::XPath(r"/html/body/div[30]/div/div/div/div[2]/span/div/span[1]/span/span")).await.expect("could not find: btn_login_with_mail");
    // btn_login_with_mail.click().await.unwrap();

    // let btn_login_with_mail = driver.find(By::ClassName("web_ui__Text__text web_ui__Text__body web_ui__Text__left web_ui__Text__primary web_ui__Text__underline")).await.expect("could not find: btn_login_with_mail");
    // btn_login_with_mail.click().await.unwrap();

    // let btn_login_with_mail = driver.find(By::LinkText("Log in") ).await.expect("could not find: btn_login_with_mail");
    // btn_login_with_mail.click().await.unwrap();

    // let btn_login_with_mail = driver.find(By::PartialLinkText("Log in") ).await.expect("could not find: btn_login_with_mail");
    // // e zashto in the fuck ne raboti s `LinkText`
    // btn_login_with_mail.click().await.unwrap();

    // let active_element = driver.active_element().await.unwrap();
    // dbg!(&active_element);
    // active_element.send_keys("  ").await.unwrap();

    // let window = driver.find(By::ClassName("web_ui__Navigation__content")).await.unwrap();
    // let text = window.text().await.unwrap();
    // dbg!(text);
    // // let btn_login_with_mail = window.find(By::).await.expect("could not find: btn_login_with_mail");
    // // window.send_keys("  ").await.unwrap();

    // driver.find_all(by)
    // driver.get_window_rect()
    // driver.set_script_timeout(time_to_wait)

    // E WTF TOVA RABOTI
    // let idk = driver.find(By::XPath("/html/body/div[11]/div/div/div/div[2]/span/div/span[1]/span/span")).await.unwrap();
    // let text = idk.text().await.unwrap();
    // dbg!(text);

    let btn_already_have_an_account_login = driver.find(By::XPath("/html/body/div[11]/div/div/div/div[2]/span/div/span[2]/span/span")).await.unwrap();
    btn_already_have_an_account_login.click().await.unwrap();

    let btn_or_login_with_email = driver.find(By::XPath("/html/body/div[11]/div/div/div/div[2]/span/div/span[1]/span/span")).await.unwrap();
    btn_or_login_with_email.click().await.unwrap();

    let inp_email_or_username = driver.find(By::XPath(r#"//*[@id="username"]"#)).await.unwrap();
    inp_email_or_username.send_keys("SOME_EMAIL").await.unwrap();

    let inp_password = driver.find(By::XPath(r#"//*[@id="password"]"#)).await.unwrap();
    inp_password.send_keys("SOME_PASSWORD\n").await.unwrap();

    { // hui

        //  let elem_form = driver.find(By::Id("search-form")).await.unwrap();

        //  // Find element from element.
        //  let elem_text = elem_form.find(By::Id("searchInput")).await.unwrap();

        //  // Type in the search terms.
        //  elem_text.send_keys("selenium").await.unwrap();

        //  // Click the search button.
        //  let elem_button = elem_form.find(By::Css("button[type='submit']")).await.unwrap();
        //  elem_button.click().await.unwrap();

        //  // Look for header to implicitly wait for the page to load.
        //  driver.find(By::ClassName("firstHeading")).await.unwrap();
        //  assert_eq!(driver.title().await.unwrap(), "Selenium - Wikipedia");
    }

    pause();

    driver.quit().await.unwrap();
}
