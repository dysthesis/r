use chrono::{DateTime, Utc};
use serde::{Serialize, ser::SerializeSeq};
use thiserror::Error;
use url::Url;

pub type Time = DateTime<Utc>;

pub trait HasId {
    fn get_id(&self) -> String;
}

pub trait HasUrl {
    fn get_url(&self) -> Result<Option<Url>, url::ParseError>;
}

pub trait HasItems {
    type Item: ToArticle;
    fn get_items(&self) -> Vec<Self::Item>;
}

pub trait HasTitle {
    fn get_title(&self) -> String;
}

pub trait HasContent {
    fn get_content(&self) -> Option<String>;
}

pub trait HasAuthor {
    fn get_author(&self) -> String;
}

pub trait HasSummary {
    fn get_summary(&self) -> Option<String>;
}

pub trait HasPublishedTime {
    fn get_published_time(&self) -> Result<Option<Time>, chrono::ParseError>;
}
pub trait HasUpdatedTime {
    fn get_updated_time(&self) -> Result<Option<Time>, chrono::ParseError>;
}

pub trait ToArticle {
    fn to_article(&self) -> Result<Article, ArticleError>;
}

#[derive(Debug, Clone, Serialize)]
pub struct Article {
    id: String,

    /// The direct URL to the article on the web.
    url: Option<Url>,

    /// The title of the article.
    title: String,

    /// The authors of the article.
    author: String,

    /// The HTML content of the article.
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,

    /// A short summary or description of the article.
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,

    /// The date and time the article was originally published.
    #[serde(skip_serializing_if = "Option::is_none")]
    published_at: Option<Time>,

    /// The date and time the article was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<Time>,
}

impl<T> ToArticle for T
where
    T: HasId
        + HasUrl
        + HasTitle
        + HasAuthor
        + HasContent
        + HasSummary
        + HasPublishedTime
        + HasUpdatedTime,
{
    fn to_article(&self) -> Result<Article, ArticleError> {
        let id = self.get_id();
        let url = self.get_url().map_err(|e| ArticleError::FailedToParseUrl {
            context: UrlContext::EntryUrl,
            reason: e,
        })?;
        let title = self.get_title();
        let author = self.get_author();
        let content = self.get_content();
        let summary = self.get_summary();
        let published_at = self
            .get_published_time()
            .map_err(|e| ArticleError::FailedToParseTime { reason: e })?;
        let updated_at = self
            .get_updated_time()
            .map_err(|e| ArticleError::FailedToParseTime { reason: e })?;
        Ok(Article {
            id,
            url,
            title,
            author,
            content,
            summary,
            published_at,
            updated_at,
        })
    }
}

#[derive(Debug, Error)]
pub enum ArticleError {
    #[error("Failed to parse item: {item:?}")]
    FailedToParseItem { item: FeedItem },
    #[error("Failed to parse {context:?} URL: {reason:?}")]
    FailedToParseUrl {
        context: UrlContext,
        reason: url::ParseError,
    },
    #[error("Failed to parse time: {reason}")]
    FailedToParseTime { reason: chrono::ParseError },
}

#[derive(Debug)]
pub enum FeedItem {
    Rss(Box<rss::Item>),
    Atom(Box<atom_syndication::Entry>),
}

impl From<rss::Item> for FeedItem {
    fn from(value: rss::Item) -> Self {
        FeedItem::Rss(Box::new(value))
    }
}
impl From<atom_syndication::Entry> for FeedItem {
    fn from(value: atom_syndication::Entry) -> Self {
        FeedItem::Atom(Box::new(value))
    }
}

#[derive(Debug)]
pub enum UrlContext {
    FeedUrl,
    EntryUrl,
}

pub struct Articles {
    /// The URL of the source feed this article came from.
    source_url: Option<Url>,

    /// The title of the source feed.
    source_title: Option<String>,

    articles: Vec<Article>,
}
impl Articles {
    pub fn parse<T, I>(source: T) -> Result<Self, ArticleError>
    where
        T: HasTitle + HasUrl + HasItems<Item = I>,
        I: ToArticle + Into<FeedItem>,
    {
        let items: Vec<I> = source.get_items();
        let source_url = source
            .get_url()
            .map_err(|e| ArticleError::FailedToParseUrl {
                context: UrlContext::FeedUrl,
                reason: e,
            })?;

        let source_title = Some(source.get_title());

        let mut articles = Vec::with_capacity(items.len());
        for item in items {
            let parsed: Article = item.to_article()?;
            articles.push(parsed);
        }

        Ok(Self {
            source_title,
            source_url,
            articles,
        })
    }
}

impl Serialize for Articles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Item<'a> {
            #[serde(flatten)]
            article: &'a Article,
            #[serde(skip_serializing_if = "Option::is_none")]
            source_url: &'a Option<Url>,
            #[serde(skip_serializing_if = "Option::is_none")]
            source_title: &'a Option<String>,
        }

        let mut seq = serializer.serialize_seq(Some(self.articles.len()))?;

        for art in &self.articles {
            let item = Item {
                article: art,
                source_url: &self.source_url,
                source_title: &self.source_title,
            };
            seq.serialize_element(&item)?;
        }

        seq.end()
    }
}
