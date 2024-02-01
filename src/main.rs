mod atom;
mod feed;
mod summary;

#[derive(Debug)]
struct FeedDetails<'a> {
    kind: &'a str,
    url: &'a str,
    category: &'a str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        FeedDetails {
            kind: "atom",
            url: "https://go.dev/blog/feed.atom",
            category: "golang",
        },
        FeedDetails {
            kind: "atom",
            url: "https://blog.rust-lang.org/feed.xml",
            category: "rust",
        },
        FeedDetails {
            kind: "feed",
            url: "https://blogs.gnome.org/uraeus/feed/",
            category: "rust",
        },
        FeedDetails {
            kind: "feed",
            url: "https://words.filippo.io/rss/",
            category: "golang",
        },
    ];

    for url in urls {
        let response = match url.kind {
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
