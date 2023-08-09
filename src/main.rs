use headless_chrome::Browser;
use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::{copy, Write};
use std::{error::Error, io};

async fn download_video_url(url: &str) -> Result<(), Box<dyn Error>> {
    print!("\nextracting... \n");
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    tab.navigate_to(url)?;
    tab.wait_for_element("video")?;
    let video_src = tab.evaluate("document.querySelector('video').src", true)?;
    if let Some(json_value) = video_src.value {
        if let Value::String(url) = json_value {
            println!("downloading...\n");
            let response = reqwest::get(url).await?;
            if response.status().is_success() {
                let mut dest_file = File::create("video.mp4")?;
                let content = response.bytes().await?;
                copy(&mut content.as_ref(), &mut dest_file)?;
                println!("download complete. press enter key to exit.");
                let _ = io::stdin().read_line(&mut String::new());
            } else {
                println!("failed to download video: {:?}", response.status());
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("gm — this cli is for downloading videos from instagram reels, threads or twitter/x. hope u enjoy find it useful <3 \n");
    let mut input = String::new();
    println!("enter url: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let video_url = input.trim();
    if video_url.is_empty() {
        println!("no message entered. exiting...");
    }
    let video_url = input.trim();
    if let Err(err) = download_video_url(video_url).await {
        eprintln!("error: {}", err);
    }
}
