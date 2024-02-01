mod atom;
mod feed;
mod summary;

use serde_derive::Deserialize;
use std::sync::Arc;
use std::thread::JoinHandle;
use tokio::sync::Semaphore;
use tokio::task;

#[derive(Deserialize, Debug)]
struct FeedDetails {
    kind: String,
    url: String,
    category: String,
}

use serde_json;
use std::fs::File;
use std::io::Read;

fn read_config() -> std::io::Result<Vec<FeedDetails>> {
    let mut file = File::open("config.json")?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let urls: Vec<FeedDetails> = serde_json::from_str(&json_data)?;

    Ok(urls)
}

async fn process_url(url: &FeedDetails) -> Option<String> {
    println!("Processing url");
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

async fn reader() -> Result<(), Box<dyn std::error::Error>> {
    let urls = read_config()?;
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
    reader().await
}
