#![feature(box_into_inner)]
use std::{env, fs};

use crate::{
    article::Articles,
    feed_parser::{FeedParser, FeedParserError},
};

mod article;
mod atom;
mod feed_parser;
mod rss;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    // We expect the only argument to be the path to the feed file.
    let path = args
        .get(1)
        .expect("Could not find the path to the feed file in the command-line arguments!");
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Failed to open the file {path}: {e}"))?;

    let feed: FeedParser = contents
        .as_str()
        .try_into()
        .map_err(|val: FeedParserError| val.to_string())?;

    let articles = feed.parse().map_err(|e| e.to_string())?;
    let json = serde_json::to_string(&articles).map_err(|e| e.to_string())?;

    println!("{}", json);
    Ok(())
}
