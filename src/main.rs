#![feature(box_into_inner)]
use std::io::{self, Read};

use crate::feed_parser::{FeedParser, FeedParserError};

mod article;
mod atom;
mod feed_parser;
mod rss;

fn main() -> Result<(), String> {
    let mut contents = String::new();
    let _ = io::stdin()
        .read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;

    let feed: FeedParser = contents
        .as_str()
        .try_into()
        .map_err(|val: FeedParserError| val.to_string())?;

    let articles = feed.parse().map_err(|e| e.to_string())?;
    let json = serde_json::to_string(&articles).map_err(|e| e.to_string())?;

    println!("{json}");
    Ok(())
}
