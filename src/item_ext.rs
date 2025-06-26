use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use atom_syndication::Entry;
use rss::Item;

/// Trait because we cannot add `impl Hash` directly to foreign types.
pub trait Hashable {
    /// Returns a 16-hex-digit (64-bit) string.
    fn hash(&self) -> String;
}

/// Helper to avoid repeating the finish/format boilerplate.
fn finish(h: DefaultHasher) -> String {
    format!("{:016x}", h.finish())
}

impl Hashable for Item {
    fn hash(&self) -> String {
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

impl Hashable for Entry {
    fn hash(&self) -> String {
        let link = self.links().first().map(|l| l.href()).unwrap_or_default();
        let title = self.title(); // ATOM <title>
        let stamp = self
            .updated() // always present
            .to_string();

        let mut h = DefaultHasher::new();
        link.hash(&mut h);
        title.hash(&mut h);
        stamp.hash(&mut h);
        finish(h)
    }
}
