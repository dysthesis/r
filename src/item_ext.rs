use atom_syndication::Entry;
use rss::Item;

// NOTE: We do this because the orphan rule prevents us from implementing `Hash` for foreign types.
pub trait Hashable {
    fn hash(&self) -> String;
}

impl Hashable for Item {
    fn hash(&self) -> String {
        todo!()
    }
}

impl Hashable for Entry {
    fn hash(&self) -> String {
        todo!()
    }
}
