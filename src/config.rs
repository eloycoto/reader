use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FeedKind {
    Feed,
    Atom,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FeedDetails {
    pub kind: FeedKind,
    pub url: String,
    pub category: String,
}

impl FeedDetails {
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn kind(&self) -> FeedKind {
        self.kind
    }

    pub fn as_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
