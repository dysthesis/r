use std::{env, fs, path::PathBuf, process};

use url::Url;

use crate::{
    article::{Article, FullText, SummaryOnly},
    content::HttpContent,
    feed::FeedParser,
};

mod article;
mod content;
mod feed;
mod item_ext;
mod url_ext;

const LIMIT: usize = 32;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // Print an error message to standard error.
        eprintln!("Usage: {} <path1> <path2>", args[0]);
        // Exit the program with a non-zero status code to indicate an error.
        process::exit(1);
    }
    let path = PathBuf::from(&args[1]);
    let feeds_file = fs::read_to_string(path)?;
    let feeds: Vec<Url> = feeds_file
        .lines()
        .filter_map(|url| Url::parse(url).ok())
        .collect();

    let client = surf::client();
    let fetched: Vec<HttpContent> =
        smol::block_on(async { HttpContent::fetch(LIMIT, feeds, &client).await })
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
            let articles: Vec<Article<SummaryOnly>> = feed.into();
            smol::block_on(async {
                Article::<SummaryOnly>::upgrade(articles, &client, LIMIT).await
            })
        })
        .filter_map(|val| val.ok())
        .collect();
    println!("{}", serde_json::to_string(&articles)?);
    Ok(())
}
