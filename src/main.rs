mod atom;
mod config;
mod feed;
mod get;
mod summary;

use chrono::Local;
use clap::{arg, command, Command};
use env_logger;
use log::info;
use serde_json;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::task;

static CONFIG_FILE: &str = "config.json";

fn read_config(config: &String) -> std::io::Result<Vec<config::FeedDetails>> {
    let mut file = File::open(config)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let urls = serde_json::from_str(&json_data)?;

    Ok(urls)
}

async fn process_url(url: &config::FeedDetails, days: i64) -> Option<(String, String)> {
    let feed = get::check_feed(url.url(), url.kind()).await.ok()?;

    match feed.as_markdown(days) {
        Some(data) => Some((url.category.clone(), data)),
        None => {
            info!("Feed with url '{}' has no new entries", url.url.to_string());
            None
        }
    }
}

async fn reader(
    config: &String,
    days: i64,
    output: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let urls = read_config(config)?;
    let sem = Arc::new(Semaphore::new(10));
    let mut res = Vec::new();
    let feeds = Arc::new(Mutex::new(Vec::new()));
    for url in urls {
        let permit = Arc::clone(&sem).acquire_owned().await;
        let feeds_clone = Arc::clone(&feeds);
        let handle = task::spawn(async move {
            let _permit = permit;

            if let Some((cat, response)) = process_url(&url, days).await {
                feeds_clone.lock().unwrap().push((cat, response));
            }
        });
        res.push(handle);
    }

    for result in res {
        result.await.unwrap();
    }

    feeds.lock().unwrap().sort_by(|a, b| a.0.cmp(&b.0));

    if feeds.lock().unwrap().is_empty() {
        println!("No feeds today, bye!");
        return Ok(());
    }

    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let file_name = format!("{}/output_{}.md", output, current_date);
    let mut file = File::create(&file_name)?;
    file.write_all(format!("# Entries for {}", current_date).as_bytes())?;
    for feed in feeds.lock().unwrap().iter() {
        file.write_all(feed.1.as_bytes())?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut command = command!()
        .subcommand(
            Command::new("run")
                .about("read all feeds and dump it")
                .arg(
                    arg!(
                        -c --config <FILE> "Sets a custom config file"
                    )
                    .id("config")
                    .required(false),
                )
                .arg(
                    arg!(
                        -o --output <dir> "output dir"
                    )
                    .id("output")
                    .required(false),
                )
                .arg(
                    arg!(
                        -d --days <number> "the number of days to check"
                    )
                    .id("days")
                    .required(false),
                ),
        )
        .subcommand(
            Command::new("check")
                .about("check an url")
                .arg(
                    arg!(
                        -u --url <url> "url to check"
                    )
                    .id("url")
                    .required(true),
                )
                .arg(
                    arg!(
                        -t --type <string> "type to be used 'feed' or 'atom'"
                    )
                    .id("type")
                    .required(true),
                ),
        )
        .subcommand(
            Command::new("get_feed").about("get a feed from url").arg(
                arg!(
                    -u --url <url> "url to check"
                )
                .id("url")
                .required(true),
            ),
        );

    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            let days: Option<i64> = run_matches
                .get_one::<String>("days")
                .map(|s| s.parse())
                .transpose()?;

            let default_config = CONFIG_FILE.to_string();
            let default_output = "output".to_string();

            let output_dir = run_matches
                .get_one::<String>("output")
                .unwrap_or(&default_output);
            fs::create_dir_all(output_dir)?;
            let config_path = run_matches
                .get_one::<String>("config")
                .unwrap_or(&default_config);
            return reader(config_path, days.unwrap_or(1), output_dir).await;
        }

        Some(("check", check_matches)) => {
            let default_type = "atom".to_string();
            let feed_type_str = check_matches
                .get_one::<String>("type")
                .unwrap_or(&default_type);

            let feed_type = match feed_type_str.as_str() {
                "atom" => config::FeedKind::Atom,
                "feed" => config::FeedKind::Feed,
                _ => return Err("No valid feed type".into()),
            };

            let url = check_matches.get_one::<String>("url");
            if url.is_none() {
                return Err("No valid url".into());
            }

            let _ = get::check_feed(url.unwrap(), feed_type).await?;

            let result = config::FeedDetails {
                kind: feed_type,
                url: url.unwrap().to_string(),
                category: "".to_string(),
            };

            println!("{}", result.as_json()?);
            Ok(())
        }
        Some(("get_feed", get_feed)) => {
            let url = get_feed.get_one::<String>("url").unwrap();
            let res = get::get_feeds_urls(url.to_string()).await?;

            for feed_details in res.iter() {
                match get::check_feed(feed_details.url(), feed_details.kind()).await {
                    Ok(_) => {
                        let result = config::FeedDetails {
                            kind: config::FeedKind::Atom,
                            url: feed_details.url().to_string(),
                            category: "".to_string(),
                        };
                        println!("{}", result.as_json()?);
                    }
                    Err(_) => continue,
                }
            }
            Ok(())
        }
        _ => {
            command.print_help()?;
            Err("Invalid command".into())
        }
    }
}
