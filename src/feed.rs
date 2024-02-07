use chrono::DateTime;
use rss::Channel;
use std::fmt;

use crate::summary;

#[derive(Debug)]
pub enum FeedError {
    ConnectionError,
    StatusError,
    ReadError,
    RSSParserError,
}

impl std::error::Error for FeedError {}

impl fmt::Display for FeedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FeedError:: {:?}", self)
    }
}

pub struct Feed {
    url: String,
}

impl Feed {
    pub fn new(url: String) -> Feed {
        Feed { url }
    }

    pub async fn parse_feed(&self) -> Result<summary::Summary, FeedError> {
        let response = reqwest::get(self.url.as_str())
            .await
            .map_err(|_| FeedError::ConnectionError)?;

        if response.status() != 200 {
            log::error!(
                "Cannot get url '{}' with status {}",
                self.url,
                response.status()
            );
            return Err(FeedError::StatusError);
        }

        let content = response.bytes().await.map_err(|_| FeedError::ReadError)?;
        let channel = Channel::read_from(&content[..]).map_err(|_| FeedError::RSSParserError)?;
        Ok(self.export_summary(channel))
    }

    pub fn export_summary(&self, channel: Channel) -> summary::Summary {
        let items = channel
            .items
            .iter()
            .map(|e| {
                let date = DateTime::parse_from_rfc2822(e.pub_date().unwrap())
                    .unwrap()
                    .to_utc();
                summary::Article::new(
                    e.title().unwrap_or("Invalid").to_string(),
                    e.link().unwrap_or("Invalid").to_string(),
                    date,
                )
            })
            .collect();
        summary::Summary::new(channel.title(), channel.link(), items)
    }
}
