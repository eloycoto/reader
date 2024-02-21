use crate::feed::get_url_content;
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
        let content = get_url_content(&self.url).await?;

        let feed = Feed::read_from(&content[..]).map_err(|_| FeedError::RSSParserError)?;

        Ok(self.export_summary(feed))
    }

    pub fn export_summary(&self, feed: Feed) -> summary::Summary {
        let items = feed
            .entries()
            .iter()
            .map(|e| {
                let link = e.links()[0].href().to_string();
                let date = e.published().unwrap_or(e.updated()).to_utc();
                summary::Article::new(e.title().to_string(), link.to_string(), date)
            })
            .collect();

        summary::Summary::new(feed.title(), feed.id(), items)
    }
}
