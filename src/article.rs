use std::marker::PhantomData;

use readability::extractor;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{feed::FeedParser, item_ext::Hashable, url_ext::HasUrl};

// States for the article
pub trait ArticleState {}

#[derive(Debug)]
pub struct SummaryOnly {}
impl ArticleState for SummaryOnly {}

#[derive(Debug)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    source_url: Option<Url>,

    /// The title of the source feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    source_title: Option<String>,

    /// The direct URL to the article on the web.
    url: Option<Url>,

    /// The title of the article.
    title: String,

    /// The authors of the article.
    author: String,

    /// The HTML content of the article.
    content: String,

    /// A short summary or description of the article.
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,

    /// The date and time the article was originally published.
    #[serde(skip_serializing_if = "Option::is_none")]
    published_at: Option<String>,

    /// The date and time the article was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<String>,

    state: PhantomData<State>,
}

// Fetch the full text
impl TryFrom<Article<SummaryOnly>> for Article<FullText> {
    type Error = anyhow::Error;

    fn try_from(value: Article<SummaryOnly>) -> Result<Self, Self::Error> {
        let Article {
            id,
            source_url,
            source_title,
            url,
            title,
            author,
            mut content,
            summary,
            published_at,
            updated_at,
            state: _,
        } = value;
        if let Some(url) = url.clone() {
            let agent = ureq::Agent::new_with_defaults();
            let html = agent
                .get(url.to_string())
                .call()?
                .body_mut()
                .read_to_string()?;
            let article = extractor::extract(&mut html.as_bytes(), &url)?;
            let parsed = html2md::rewrite_html(article.content.as_str(), false);
            content = parsed
        }
        Ok(Article {
            id,
            source_url,
            source_title,
            url,
            title,
            author,
            content,
            summary,
            published_at,
            updated_at,
            state: std::marker::PhantomData::<FullText>,
        })
    }
}

impl From<FeedParser> for Vec<Article<SummaryOnly>> {
    fn from(value: FeedParser) -> Self {
        // We haven't fetched the full text content yet
        let state = PhantomData::<SummaryOnly>;
        match value {
            FeedParser::Rss(channel) => {
                // Get the feed information
                let source_title = Some(channel.title().to_string());
                let source_url = channel.get_url();

                // TODO: I'm ignoring errors for now just to get this working first. We don't want
                // to fail the entire process just because of a few parsing errors on a couple of
                // articles, but we also don't want to ignore the errors, so we should log them!
                channel
                    .items()
                    .iter()
                    .map(|item| {
                        let id = item
                            .guid()
                            .map(|val| val.value().to_string())
                            .unwrap_or(item.hash());
                        let url = item.get_url();
                        let title = item.title().unwrap_or_default().to_string();
                        let author = item.author().unwrap_or_default().to_string();
                        let content = item.content().unwrap_or_default().to_string();
                        let summary = item.description().map(|x| x.to_string());
                        let published_at = item.pub_date().map(|val| val.to_string());
                        let updated_at = None;
                        Article {
                            state,
                            source_url: source_url.clone(),
                            source_title: source_title.clone(),
                            id,
                            url,
                            title,
                            author,
                            content,
                            summary,
                            published_at,
                            updated_at,
                        }
                    })
                    .collect()
            }
            FeedParser::Atom(feed) => {
                let source_title = Some(feed.title().to_string());
                let source_url = feed.get_url();

                feed.entries()
                    .iter()
                    .map(|entry| {
                        let id = entry.id().to_string();
                        // TODO: Error handling
                        let content = entry.content().expect("an entry to have content");
                        let url = entry.get_url();
                        let title = entry.title().to_string();
                        let author: String = entry
                            .authors()
                            .iter()
                            .map(|author| author.name().to_string())
                            .collect();

                        let content = content.value().unwrap_or_default().to_string();
                        let summary = entry.summary().map(|val| val.to_string());
                        let published_at = entry.published().map(|val| val.to_string());
                        let updated_at = Some(entry.updated().to_string());
                        Article {
                            state,
                            source_url: source_url.clone(),
                            source_title: source_title.clone(),
                            id,
                            url,
                            title,
                            author,
                            content,
                            summary,
                            published_at,
                            updated_at,
                        }
                    })
                    .collect()
            }
        }
    }
}
