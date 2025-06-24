use std::{env, error::Error, fs, path::PathBuf, process};

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
    let feeds: Vec<&str> = feeds_file.lines().collect();
    let fetched: Vec<HttpContent> = feeds
        .into_iter()
        .filter_map(|val| url::Url::parse(val).ok()?.try_into().ok())
        .collect();
    let parsed: Vec<FeedParser> = fetched
        .into_iter()
        .filter_map(|val| val.try_into().ok())
        .collect();
    let articles: Vec<Article<FullText>> = parsed
        .into_iter()
        .flat_map(|feed| {
            let articles: Vec<Article<SummaryOnly>> = feed.into();
            articles
                .into_iter()
                .filter_map(|article| article.try_into().ok())
                .collect::<Vec<Article<FullText>>>()
        })
        .collect();
    println!("{}", serde_json::to_string(&articles)?);
    Ok(())
}
