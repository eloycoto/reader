mod atom;
mod feed;
mod summary;

use clap::{arg, command, Command};
use serde_derive::Deserialize;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

static CONFIG_FILE: &str = "config.json";

#[derive(Deserialize, Debug)]
struct FeedDetails {
    kind: String,
    url: String,
    category: String,
}

fn read_config(config: &String) -> std::io::Result<Vec<FeedDetails>> {
    let mut file = File::open(config)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let urls: Vec<FeedDetails> = serde_json::from_str(&json_data)?;

    Ok(urls)
}

async fn process_url(url: &FeedDetails) -> Option<String> {
    let response = match url.kind.as_str() {
        "atom" => {
            let feed = atom::Atom::new(url.url.to_string());
            let data = feed.parse_feed().await.ok()?;
            Some(data.as_markdown(200))
        }

        "feed" => {
            let feed = feed::Feed::new(url.url.to_string());
            let data = feed.parse_feed().await.ok()?;
            Some(data.as_markdown(200))
        }

        _ => None,
    };
    response.unwrap()
}

async fn reader(config: &String) -> Result<(), Box<dyn std::error::Error>> {
    let urls = read_config(config)?;
    let sem = Arc::new(Semaphore::new(10));
    let mut res = Vec::new();

    for url in urls {
        let permit = Arc::clone(&sem).acquire_owned().await;
        let handle = task::spawn(async move {
            let _permit = permit;
            if let Some(response) = process_url(&url).await {
                println!("{}", response);
            }
        });
        res.push(handle);
    }

    for result in res {
        result.await.unwrap();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .subcommand(
            Command::new("run").about("read all feeds and dump it").arg(
                arg!(
                    -c --config <FILE> "Sets a custom config file"
                )
                .id("config")
                .required(false),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            let default_config = CONFIG_FILE.to_string();
            let config_path = run_matches
                .get_one::<String>("config")
                .unwrap_or(&default_config);
            return reader(config_path).await;
        }
        _ => Err("No valid command".into()),
    }
}
