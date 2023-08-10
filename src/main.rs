use headless_chrome::Browser;
use owo_colors::OwoColorize;
use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::{copy, Write};
use std::{error::Error, io};

async fn download_video_url(url: &str) -> Result<(), Box<dyn Error>> {
    println!("\n{}", "[2/4] 🥤 extracting...".yellow());

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    tab.navigate_to(url)?;
    tab.wait_for_element("video")?;

    let video_src = tab.evaluate("document.querySelector('video').src", true)?;

    if let Some(json_value) = video_src.value {
        if let Value::String(url) = json_value {
            println!("\n{}", "[3/4] 🚚 downloading...".yellow());

            let response = reqwest::get(url).await?;

            if response.status().is_success() {
                print!("\n{}", "[4/4] 📂 file name: ".bright_cyan());

                io::stdout().flush().unwrap();
                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let file_url = input.trim().to_owned() + ".mp4";
                let mut dest_file = File::create(file_url)?;
                let content = response.bytes().await?;

                copy(&mut content.as_ref(), &mut dest_file)?;
                println!(
                    "\n{}",
                    "download complete. press enter key to exit.".bright_green()
                );

                let _ = io::stdin().read_line(&mut String::new());
            } else {
                println!("\n{}", "failed to download video".red());
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("\n{}","gm — this cli is for downloading videos from instagram reels, threads or twitter/x. hope you find it useful 💛".bright_magenta());

    print!("\n{}", "[1/4] 🔗 url: ".bright_cyan());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let video_url = input.trim();

    if let Err(err) = download_video_url(&video_url).await {
        eprintln!(
            "\n{}: {}",
            "error".bright_red(),
            err.to_string().to_lowercase()
        );
    }
}
