use atom_syndication::Entry;
use chrono::Utc;
use url::Url;

use crate::article::{
    HasAuthor, HasContent, HasId, HasItems, HasPublishedTime, HasSummary, HasTitle, HasUpdatedTime,
    HasUrl, Time,
};

impl HasId for Entry {
    fn get_id(&self) -> String {
        self.id().to_string()
    }
}

impl HasUrl for Entry {
    fn get_url(&self) -> Result<Option<url::Url>, url::ParseError> {
        let content = match self.content() {
            Some(res) => res,
            None => return Ok(None),
        };
        let link = content
            .base()
            .or(content.src())
            .or(self.links().first().map(|val| val.href()));
        match link {
            Some(link) => Ok(Some(Url::parse(link)?)),
            None => Ok(None),
        }
    }
}

impl HasContent for Entry {
    fn get_content(&self) -> Option<String> {
        self.content()
            .and_then(|val| val.value().map(|val| val.to_string()))
    }
}

impl HasUpdatedTime for Entry {
    fn get_updated_time(&self) -> Result<Option<Time>, chrono::ParseError> {
        let res: Time = self.updated().with_timezone(&Utc);
        Ok(Some(res))
    }
}

impl HasPublishedTime for Entry {
    fn get_published_time(&self) -> Result<Option<Time>, chrono::ParseError> {
        let res: Option<Time> = self.published().map(|time| time.with_timezone(&Utc));
        Ok(res)
    }
}

impl HasAuthor for Entry {
    fn get_author(&self) -> String {
        self.authors()
            .iter()
            .map(|author| author.name().to_string())
            .collect()
    }
}

impl HasSummary for Entry {
    fn get_summary(&self) -> Option<String> {
        self.summary().map(|val| val.to_string())
    }
}

impl HasTitle for Entry {
    fn get_title(&self) -> String {
        self.title().to_string()
    }
}

impl HasItems for atom_syndication::Feed {
    type Item = Entry;

    fn get_items(&self) -> Vec<Self::Item> {
        self.entries.clone()
    }
}
