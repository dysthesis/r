use std::hash::{DefaultHasher, Hash, Hasher};

use chrono::{DateTime, Utc};
use url::Url;

use crate::article::{
    HasAuthor, HasContent, HasId, HasItems, HasPublishedTime, HasSummary, HasTitle, HasUpdatedTime,
    HasUrl, Time,
};

impl HasUpdatedTime for rss::Item {
    fn get_updated_time(&self) -> Result<Option<crate::article::Time>, chrono::ParseError> {
        // NOTE: RSS does not have an updated time, so we'll just use the published time
        self.get_published_time()
    }
}

impl HasPublishedTime for rss::Item {
    fn get_published_time(&self) -> Result<Option<crate::article::Time>, chrono::ParseError> {
        match self.pub_date() {
            Some(pub_date) => Ok(Some(
                DateTime::parse_from_rfc2822(pub_date).map(|val| val.with_timezone(&Utc))?,
            )),
            None => Ok(None),
        }
    }
}

impl HasSummary for rss::Item {
    fn get_summary(&self) -> Option<String> {
        self.description.clone()
    }
}

impl HasContent for rss::Item {
    fn get_content(&self) -> Option<String> {
        self.content.clone()
    }
}

impl HasAuthor for rss::Item {
    fn get_author(&self) -> String {
        self.author.clone().unwrap_or_default()
    }
}

impl HasTitle for rss::Item {
    fn get_title(&self) -> String {
        self.title.clone().unwrap_or_default()
    }
}

impl HasUrl for rss::Item {
    fn get_url(&self) -> Result<Option<url::Url>, url::ParseError> {
        match self.link() {
            Some(link) => Ok(Some(Url::parse(link)?)),
            None => Ok(None),
        }
    }
}

impl HasId for rss::Item {
    fn get_id(&self) -> String {
        /// Helper to avoid repeating the finish/format boilerplate.
        fn finish(h: DefaultHasher) -> String {
            format!("{:016x}", h.finish())
        }
        let link = self.link().unwrap_or_default();
        let title = self.title().unwrap_or_default();
        let pub_date = self.pub_date().unwrap_or_default(); // RFC 822/1123

        let mut h = DefaultHasher::new();
        link.hash(&mut h);
        title.hash(&mut h);
        pub_date.hash(&mut h);
        finish(h)
    }
}

impl HasItems for rss::Channel {
    type Item = rss::Item;
    fn get_items(&self) -> Vec<Self::Item> {
        self.items.clone()
    }
}
