pub mod languages;

use reqwest::IntoUrl;
use serde::Deserialize;

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
