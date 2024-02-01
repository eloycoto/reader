use crate::feed::FeedError;
use crate::summary;
use atom_syndication::Feed;

pub struct Atom {
    url: String,
}

impl Atom {
    pub fn new(url: String) -> Atom {
        Atom { url }
    }

    pub async fn parse_feed<'a>(&'a self) -> Result<summary::Summary, FeedError> {
        let response = reqwest::get(self.url.clone())
            .await
            .map_err(|_| FeedError::ConnectionError)?;
        if response.status() != 200 {
            return Err(FeedError::StatusError);
        }

        let content = response.bytes().await.map_err(|_| FeedError::ReadError)?;
        let feed = Feed::read_from(&content[..]).map_err(|_| FeedError::RSSParserError)?;

        Ok(self.export_summary(feed))
    }

    pub fn export_summary(&self, feed: Feed) -> summary::Summary {
        let items = feed
            .entries()
            .iter()
            .map(|e| {
                let link = format!("{:?}", e.links()[0].href());
                let date = e.published().unwrap_or(e.updated()).to_utc();
                summary::Article::new(e.title().to_string(), link.to_string(), date)
            })
            .collect();

        summary::Summary::new(feed.title(), feed.id(), items)
    }
}
