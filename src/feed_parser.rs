use thiserror::Error;

use crate::article::{ArticleError, Articles};

#[derive(Debug)]
pub enum FeedParser {
    Rss(Box<rss::Channel>),
    Atom(Box<atom_syndication::Feed>),
}

#[derive(Debug, Error)]
pub enum FeedParserError<'a> {
    #[error("This is not a valid RSS or Atom feed")]
    InvalidFeed { content: &'a str },
}

impl<'a> TryFrom<&'a str> for FeedParser {
    type Error = FeedParserError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Ok(channel) = rss::Channel::read_from(value.as_bytes()) {
            Ok(FeedParser::Rss(Box::new(channel)))
        } else if let Ok(feed) = atom_syndication::Feed::read_from(value.as_bytes()) {
            Ok(FeedParser::Atom(Box::new(feed)))
        } else {
            Err(FeedParserError::InvalidFeed { content: value })
        }
    }
}

impl FeedParser {
    pub fn parse(self) -> Result<Articles, ArticleError> {
        match self {
            FeedParser::Rss(channel) => Articles::parse(Box::into_inner(channel)),
            FeedParser::Atom(feed) => Articles::parse(Box::into_inner(feed)),
        }
    }
}
