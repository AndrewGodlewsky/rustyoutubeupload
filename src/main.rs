use std::time::Duration;

use terminal_color_builder::OutputFormatter as tcb;
use thirtyfour::prelude::*;

mod chromedriver;
mod cookies;
mod video;

use chromedriver::ChromeDriver;
use video::Video;

async fn preform_upload(webdriver: &mut WebDriver, video: &Video) -> WebDriverResult<()> {
    webdriver.goto("https://www.youtube.com").await?;
    cookies::add_cookie(webdriver).await?;

    webdriver.goto("https://www.youtube.com/upload").await?;

    // Find hidden input text box and pass text into to specify the video path.
    webdriver
        .query(By::Css("input[type='file']"))
        .wait(Duration::from_secs_f32(15.0), Duration::from_secs_f32(0.50))
        .first()
        .await?
        .send_keys(&video.path)
        .await?;

    // Add video title to textbox
    webdriver
        .query(By::Css(
            "div[class='input-container title style-scope ytcp-video-metadata-editor-basics']",
        ))
        .wait(Duration::from_secs_f32(60.0), Duration::from_secs_f32(1.0))
        .first()
        .await?
        .find(By::Css("div[id='textbox']"))
        .await?
        .send_keys(&video.title.clone().unwrap_or_default())
        .await?;

    // Add video description to textbox
    webdriver
        .query(By::Css(
            "div[class='input-container description style-scope ytcp-video-metadata-editor-basics']"
        ))
        .wait(Duration::from_secs_f32(60.0), Duration::from_secs_f32(1.0))
        .first()
        .await?
        .find(By::Css("div[id='textbox']"))
        .await?
        .send_keys(&video.description.clone().unwrap_or_default())
        .await?;

    // Checking the `not` option for Kids button
    webdriver
        .query(By::Css("div[class='input-container style-scope ytcp-video-metadata-editor-basics']"))
        .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
        .first()
        .await?
        .query (By::XPath(r"//*[@id='audience']/ytkc-made-for-kids-select/div[4]/tp-yt-paper-radio-group/tp-yt-paper-radio-button[2]"))
        .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
        .first()
        .await?
        .click()
        .await?;

    // Click the show more button
    webdriver
        .query(By::Css(
            "div[class='toggle-section style-scope ytcp-video-metadata-editor']",
        ))
        .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
        .first()
        .await?
        .query(By::XPath(r"//*[@id='toggle-button']"))
        .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
        .first()
        .await?
        .click()
        .await?;

    // Adding Tags to the textbox.
    webdriver
        .query(By::XPath(r"/html/body/ytcp-uploads-dialog/tp-yt-paper-dialog/div/ytcp-animatable[1]/ytcp-ve/ytcp-video-metadata-editor/div/ytcp-video-metadata-editor-advanced/div[5]/ytcp-form-input-container/div[1]/div/ytcp-free-text-chip-bar/ytcp-chip-bar/div/input"))
        .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
        .first()
        .await?
        .send_keys(&video.get_tags_for_text_input())
        .await?;

    //Checking if Automatic chapters checkbox is checked
    let automatic_chapters_str = webdriver
        .query(By::Css(
            "ytcp-checkbox-lit[ class='style-scope ytcp-form-checkbox']",
        ))
        .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
        .first()
        .await?
        .attr("aria-checked")
        .await?
        .unwrap();

    // dbg!(&automatic_chapters_str);

    //When automatic_chapters_str and automatic_chapters are not equal, then run!
    match (
        video.automatic_chapters.is_some(),
        (automatic_chapters_str == "true"),
        video.automatic_chapters.unwrap_or_default(),
    ) {
        (false, _, _) => {}
        (true, true, true) => {}
        (true, false, false) => {}
        (_, _, _) => {
            // Click Automatic chapters button
            webdriver
                .query(By::Css(
                    "div[class='input-container style-scope ytcp-video-metadata-editor-advanced']",
                ))
                .wait(Duration::from_secs_f32(5.0), Duration::from_secs_f32(0.10))
                .first()
                .await?
                .find(By::Css("div[id='checkbox-container']"))
                .await?
                .click()
                .await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut chromedriver = ChromeDriver::new().await;

    let caps = DesiredCapabilities::chrome();
    let mut webdriver = WebDriver::new("http://localhost:9515", caps).await?;

    // Deserialized the list of videos
    let json_data = std::fs::read_to_string("data.json")
        .expect("No data.json file found. Please create one...");
    let videos: Vec<Video> = serde_json::from_str(&json_data).unwrap();

    // preform upload proecess on every video
    for video in videos {
        let result = preform_upload(&mut webdriver, &video).await;

        match result {
            Ok(_) => println!("Video '{:?}' uploaded Successfully", &video.title),
            Err(e) => println!(
                "Video upload '{:?}' Failed! Error : {}",
                &video.title,
                tcb::new().fg().red().text(e.to_string()).print()
            ),
        }
    }

    // Always explicitly close the browser, then close the chrome driver.
    // webdriver.quit().await?;

    chromedriver.stop_driver().await;

    Ok(())
}
