use chrono::{DateTime, Duration, Utc};
use rss::Channel;
use rss::Item;
use std::fmt;

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

    pub async fn parse_feed(&self) -> Result<Channel, FeedError> {
        let response = reqwest::get(self.url.clone())
            .await
            .map_err(|_| FeedError::ConnectionError)?;

        if response.status() != 200 {
            return Err(FeedError::StatusError);
        }

        let content = response.bytes().await.map_err(|_| FeedError::ReadError)?;
        let oo = Channel::read_from(&content[..]);
        if oo.is_err() {
            println!("{:?}", oo);
            oo.map_err(|_| FeedError::RSSParserError)?;
        }
        let channel = Channel::read_from(&content[..]).map_err(|_| FeedError::RSSParserError)?;

        Ok(channel)
    }
}

#[derive(Debug)]
pub struct ChannelSummaryInfo<'a> {
    title: &'a str,
    link: &'a str,
    items: Vec<Item>,
}

impl<'a> ChannelSummaryInfo<'a> {
    pub fn as_markown(&self, days: i64) -> Option<String> {
        let since = Utc::now() - Duration::days(days) - Duration::hours(1);
        let items = self
            .items
            .iter()
            .filter(|item| match item.pub_date() {
                Some(date_str) => match DateTime::parse_from_rfc2822(date_str) {
                    Err(_) => false,
                    Ok(date) => date > since,
                },
                None => false,
            })
            .map(|item| format!("\t - [{}]({})", item.title().unwrap(), item.link().unwrap()))
            .collect::<Vec<String>>();

        if items.is_empty() {
            return None;
        }

        return Some(format!(
            "## {} \n Blog: {} \n Links {}",
            self.title,
            self.link,
            items.join("\n")
        ));
    }
}

pub trait ChannelSummary {
    fn get_latest_info(&self) -> ChannelSummaryInfo;
}

impl ChannelSummary for Channel {
    fn get_latest_info(&self) -> ChannelSummaryInfo {
        ChannelSummaryInfo {
            title: self.title(),
            link: self.link(),
            items: self.items().to_vec(),
        }
    }
}
