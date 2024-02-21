use bytes::Bytes;
use chrono::DateTime;
use rss::Channel;
use std::fmt;

use crate::summary;

const MAX_ERR_CONTENT_LEN: usize = 30;

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

pub async fn get_url_content(url: &str) -> Result<Bytes, FeedError> {
    let response = reqwest::get(url)
        .await
        .map_err(|_| FeedError::ConnectionError)?;
    let status_code = response.status();
    let headers = response.headers().clone();

    let content = response.bytes().await.map_err(|_| FeedError::ReadError)?;
    if status_code != 200 {
        let mut content_len = content.len();
        if content_len > MAX_ERR_CONTENT_LEN {
            content_len = MAX_ERR_CONTENT_LEN;
        }

        log::error!(
            "Cannot get url '{}' with status '{}' Content='{:?}' headers='{:?}'",
            url,
            status_code,
            content.slice(1..content_len),
            headers
        );
        return Err(FeedError::StatusError);
    }
    Ok(content)
}

pub struct Feed {
    url: String,
}

impl Feed {
    pub fn new(url: String) -> Feed {
        Feed { url }
    }

    pub async fn parse_feed(&self) -> Result<summary::Summary, FeedError> {
        let content = get_url_content(&self.url).await?;

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
