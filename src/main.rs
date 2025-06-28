use std::{env, fs, path::PathBuf};

use url::Url;

use crate::{
    article::{Article, FullText, SummaryOnly},
    content::HttpContent,
    feed::FeedParser,
};
use anyhow::anyhow;

mod article;
mod content;
mod feed;
mod item_ext;
mod url_ext;

const DEFAULT_MAX_CONCURRENT_FETCH: usize = 32;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = PathBuf::from(args.get(1).ok_or_else(|| {
        anyhow!(
            "Missing feeds file path!\n\nUsage: {} <FEEDS PATH> <MAX CONCURRENT FETCH>",
            args[0]
        )
    })?);
    let limit: usize = args.get(2).map_or(DEFAULT_MAX_CONCURRENT_FETCH, |v| {
        v.parse::<usize>()
            .expect("Cannot parse MAX CONCURRENT FETCH as usize")
    });
    let feeds_file = fs::read_to_string(path)?;
    let feeds: Vec<Url> = feeds_file
        .lines()
        .filter_map(|url| Url::parse(url).ok())
        .collect();

    let client = surf::client();
    let fetched: Vec<HttpContent> =
        smol::block_on(async { HttpContent::fetch(limit, feeds, &client).await })
            .into_iter()
            .filter_map(|val| val.ok())
            .collect();
    let parsed: Vec<FeedParser> = fetched
        .into_iter()
        .filter_map(|val| val.try_into().ok())
        .collect();
    let articles: Vec<Article<FullText>> = parsed
        .into_iter()
        .flat_map(|feed| {
            let articles: Vec<Article<SummaryOnly>> = feed.try_into().unwrap();
            smol::block_on(async {
                Article::<SummaryOnly>::upgrade(articles, &client, limit).await
            })
        })
        .filter_map(|val| val.ok())
        .collect();
    println!("{}", serde_json::to_string(&articles)?);
    Ok(())
}
