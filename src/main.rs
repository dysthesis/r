use std::error::Error;

use crate::{
    article::{Article, SummaryOnly},
    content::HttpContent,
    feed::FeedParser,
};

mod article;
mod content;
mod feed;
mod item_ext;
mod url_ext;

fn main() -> anyhow::Result<()> {
    let feeds = vec![
        "https://matklad.github.io/feed.xml",
        "https://nullprogram.com/feed/",
        "https://googleprojectzero.blogspot.com/feeds/posts/default",
    ];
    let fetched: Vec<HttpContent> = feeds
        .into_iter()
        .filter_map(|val| url::Url::parse(val).ok()?.try_into().ok())
        .collect();
    let parsed: Vec<FeedParser> = fetched
        .into_iter()
        .filter_map(|val| val.try_into().ok())
        .collect();
    let articles: Vec<Article<SummaryOnly>> = parsed
        .into_iter()
        .flat_map(|feed| {
            let articles: Vec<Article<SummaryOnly>> = feed.into();
            articles
        })
        .collect();
    println!("{articles:?}");
    Ok(())
}
