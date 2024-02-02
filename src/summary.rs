use chrono::{DateTime, Duration, Utc};
use log::info;

#[derive(Debug, Clone)]
pub struct Article {
    pub title: String,
    pub link: String,
    pub pub_date: DateTime<Utc>,
}

impl Article {
    pub fn new(title: String, link: String, pub_date: DateTime<Utc>) -> Article {
        Article {
            title,
            link,
            pub_date,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Summary {
    pub title: String,
    pub link: String,
    pub items: Vec<Article>,
}

impl Summary {
    pub fn new(title: &str, link: &str, items: Vec<Article>) -> Summary {
        Summary {
            title: title.to_string(),
            link: link.to_string(),
            items: items,
        }
    }

    pub fn as_markdown(&self, days: i64) -> Option<String> {
        let since = Utc::now() - Duration::days(days) - Duration::hours(2);
        let items = self
            .items
            .iter()
            .filter(|item| item.pub_date > since)
            .map(|item| format!("- [{}]({})", item.title, item.link))
            .collect::<Vec<String>>();

        if items.is_empty() {
            return None;
        }
        info!(
            "Blog '{}' has {} total entries, and {} new",
            self.link,
            self.items.len(),
            items.len()
        );
        return Some(format!(
            "\n## {} \nBlog: {} \n\n{}",
            self.title,
            self.link,
            items.join("\n")
        ));
    }
}
