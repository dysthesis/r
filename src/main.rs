use std::{env, fs};

use crate::feed_parser::FeedParser;

mod feed_parser;
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    // We expect the only argument to be the path to the feed file.
    let path = args
        .get(1)
        .expect("Could not find the path to the feed file in the command-line arguments!");
    let contents =
        fs::read_to_string(path).expect(format!("Could not read the file {path}").as_str());
    let feed: FeedParser = contents.as_str().try_into()?;
    println!("{feed:?}");
    Ok(())
}
