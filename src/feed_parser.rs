use anyhow::anyhow;

#[derive(Debug)]
pub enum FeedParser {
    Rss(rss::Channel),
    Atom(atom_syndication::Feed),
}

impl TryFrom<&str> for FeedParser {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(channel) = rss::Channel::read_from(value.as_bytes()) {
            Ok(FeedParser::Rss(channel))
        } else if let Ok(feed) = atom_syndication::Feed::read_from(value.as_bytes()) {
            Ok(FeedParser::Atom(feed))
        } else {
            Err(anyhow!("Cannot parse content as RSS or Atom!"))
        }
    }
}
