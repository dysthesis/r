use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

// States for the article
pub trait ArticleState {}

pub struct SummaryOnly {}
impl ArticleState for SummaryOnly {}

pub struct FullText {}
impl ArticleState for FullText {}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A single article
pub struct Article<State>
where
    State: ArticleState,
{
    id: String,

    /// The URL of the source feed this article came from.
    source_url: Url,

    /// The title of the source feed.
    source_title: Option<String>,

    /// The direct URL to the article on the web.
    url: Url,

    /// The title of the article.
    title: String,

    /// The authors of the article.
    authors: Vec<String>,

    /// The HTML content of the article.
    content: String,

    /// A short summary or description of the article.
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,

    /// The date and time the article was originally published.
    published_at: DateTime<Utc>,

    /// The date and time the article was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<DateTime<Utc>>,

    _state: PhantomData<State>,
}

// Fetch the full text
impl From<Article<SummaryOnly>> for Article<FullText> {
    fn from(value: Article<SummaryOnly>) -> Self {
        todo!()
    }
}
