use anyhow::anyhow;
use atom_syndication::Feed;
use rss::Channel;

use crate::content::HttpContent;
#[derive(Debug)]
pub enum FeedParser {
    Rss(Channel),
    Atom(Feed),
}

impl TryFrom<HttpContent> for FeedParser {
    type Error = anyhow::Error;

    fn try_from(value: HttpContent) -> Result<Self, Self::Error> {
        let content = value.to_string();
        if let Ok(channel) = rss::Channel::read_from(content.as_bytes()) {
            Ok(FeedParser::Rss(channel))
        } else if let Ok(feed) = atom_syndication::Feed::read_from(content.as_bytes()) {
            Ok(FeedParser::Atom(feed))
        } else {
            Err(anyhow!("Cannot parse content as RSS or Atom!"))
        }
    }
}
