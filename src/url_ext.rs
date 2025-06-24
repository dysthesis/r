use atom_syndication::{Entry, Feed};
use rss::{Channel, Item};
use url::Url;

pub trait HasUrl {
    fn get_url(&self) -> Option<Url>;
}

impl HasUrl for Entry {
    fn get_url(&self) -> Option<Url> {
        let content = self.content()?;
        let link = content
            .base()
            .or(content.src())
            .or(self.links().first().map(|val| val.href()))?;
        Url::parse(link).ok()
    }
}

impl HasUrl for Item {
    fn get_url(&self) -> Option<Url> {
        let link = self.link()?;
        Url::parse(link).ok()
    }
}

impl HasUrl for Feed {
    fn get_url(&self) -> Option<Url> {
        let link = self.base()?;
        Url::parse(link).ok()
    }
}

impl HasUrl for Channel {
    fn get_url(&self) -> Option<Url> {
        let link = self.link();
        Url::parse(link).ok()
    }
}
