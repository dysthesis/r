use atom_syndication::Feed;
use rss::Channel;

use crate::content::HttpContent;
pub enum FeedParser {
    Rss(Channel),
    Atom(Feed),
}

impl TryFrom<HttpContent> for FeedParser {
    type Error = anyhow::Error;

    fn try_from(value: HttpContent) -> Result<Self, Self::Error> {
        todo!()
    }
}
