mod atom;
mod feed;
mod summary;

use serde_derive::Deserialize;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = read_config()?;
    for url in urls {
        let response = match url.kind.as_str() {
            "atom" => {
                let feed = atom::Atom::new(url.url.to_string());
                let data = feed.parse_feed().await?;
                Some(data.as_markdown(200))
            }

            "feed" => {
                let feed = feed::Feed::new(url.url.to_string());
                let data = feed.parse_feed().await?;
                Some(data.as_markdown(200))
            }
            _ => None,
        };
        if response.is_some() {
            println!("{}", response.unwrap().unwrap());
        }
    }

    Ok(())
}
