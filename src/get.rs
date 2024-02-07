use crate::atom::Atom;
use crate::config::{FeedDetails, FeedKind};
use crate::feed::{Feed, FeedError};
use crate::summary;
use select::document::Document;
use select::predicate::Name;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GetErrors {
    InvalidURL,
    ReqwestError(reqwest::Error),
    StatusError,
    NoFeedFound,
}

impl fmt::Display for GetErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetErrors::InvalidURL => write!(f, "Invalid URL"),
            GetErrors::ReqwestError(e) => write!(f, "Reqwest error: {}", e),
            GetErrors::StatusError => write!(f, "Non-200 status code received"),
            GetErrors::NoFeedFound => write!(f, "Feed cannot be found"),
        }
    }
}

impl Error for GetErrors {}

pub async fn get_feeds_urls(url: String) -> Result<Vec<FeedDetails>, GetErrors> {
    if url.is_empty() {
        return Err(GetErrors::InvalidURL);
    }

    let response = reqwest::get(url.as_str())
        .await
        .map_err(|e| GetErrors::ReqwestError(e))?;

    if response.status() != 200 {
        log::error!("Cannot get url '{}' with status {}", url, response.status());
        return Err(GetErrors::StatusError);
    }

    let content = response
        .text()
        .await
        .map_err(|e| GetErrors::ReqwestError(e))?;

    let document = Document::from(content.as_str());

    let mut result = Vec::new();
    for node in document
        .find(Name("link"))
        .filter(|n| n.attr("type").is_some())
    {
        let node_type = node.attr("type").unwrap();
        let kind = match node_type {
            "application/rss+xml" => FeedKind::Feed,
            "application/atom+xml" => FeedKind::Atom,
            _ => continue,
        };

        if let Some(link) = node.attr("href") {
            result.push(FeedDetails {
                kind,
                url: link.to_string(),
                category: "".to_string(),
            });
        }
    }
    if result.is_empty() {
        return Err(GetErrors::NoFeedFound);
    }

    return Ok(result);
}

pub async fn check_feed(url: &str, kind: FeedKind) -> Result<summary::Summary, FeedError> {
    match kind {
        FeedKind::Atom => {
            let feed = Atom::new(url.to_string());
            let data = feed.parse_feed().await?;
            Ok(data)
        }
        FeedKind::Feed => {
            let feed = Feed::new(url.to_string());
            let data = feed.parse_feed().await?;
            Ok(data)
        }
    }
}
