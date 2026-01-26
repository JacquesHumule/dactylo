use clap::ValueEnum;
use reqwest::{IntoUrl, Url};
use serde::Deserialize;

#[derive(Debug, Clone, ValueEnum)]
pub enum Languages {
    French,
    English,
}

impl Languages {
    pub fn as_str(&self) -> &'static str {
        use Languages::*;
        match self {
            English => "english",
            French => "french",
        }
    }

    pub fn to_url(&self) -> Url {
        Url::parse(&format!(
            "https://www.monkeytype.com/languages/{}.json",
            self.as_str()
        ))
        .unwrap()
    }
}

#[derive(Debug, Clone, Deserialize)]
struct MonkeyTypeDictionary {
    words: Vec<String>,
    // name: String,
}

pub async fn fetch(url: impl IntoUrl) -> color_eyre::Result<super::Dictionary> {
    let res = reqwest::get(url).await?;
    let mt_dict: MonkeyTypeDictionary = res.json().await?;

    Ok(super::Dictionary {
        words: mt_dict.words,
        // lang: "en".to_string(),
    })
}
