mod feed;
use feed::ChannelSummary;
use feed::Feed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let url = "https://blogs.gnome.org/uraeus/feed/";
    //let url = "https://go.dev/blog/feed.atom";

    let url = "https://blog.rust-lang.org/feed.xml";
    let feed = Feed::new(url.to_string());
    let fres = feed.parse_feed().await?;

    println!("{:?}", fres.get_latest_info().as_markown(200).unwrap());

    Ok(())
}
