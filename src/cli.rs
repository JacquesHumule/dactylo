use clap::{ArgGroup, Args, Parser, Subcommand};
use clap_complete::Shell;
use reqwest::Url;

use crate::dictionary::monkeytype;

#[derive(Parser)]
#[command(version, about, group(ArgGroup::new("dictionary")))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start a test with a word count
    Words {
        /// The number of words of the test
        number: usize,

        #[command(flatten)]
        dictionary: DictionarySource,
    },

    Completions {
        shell: Shell,
    },
}

#[derive(Debug, Clone, Args)]
#[group(required = false, multiple = false)]
pub struct DictionarySource {
    /// Monkeytype language url (e.g.: <https://monkeytype.com/languages/english.json>)
    #[arg(long = "mt-url")]
    pub monkeytype_url: Option<Url>,
    /// Monkeytype language name (e.g.: english)
    #[arg(long = "mt-lang", default_value = "english")]
    pub monkeytype_lang: Option<monkeytype::languages::Languages>,
}

impl DictionarySource {
    pub fn to_enum(&self) -> DictionarySourceEnum {
        if let Some(monkeytype_url) = &self.monkeytype_url {
            DictionarySourceEnum::MonkeytypeUrl(monkeytype_url.clone())
        } else if let Some(monkeytype_lang) = &self.monkeytype_lang {
            DictionarySourceEnum::MonkeytypeLang(monkeytype_lang.clone())
        } else {
            DictionarySourceEnum::MonkeytypeLang(monkeytype::languages::Languages::English)
        }
    }
}

pub enum DictionarySourceEnum {
    MonkeytypeUrl(Url),
    MonkeytypeLang(monkeytype::languages::Languages),
}
