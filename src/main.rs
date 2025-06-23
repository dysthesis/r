use quick_xml::Reader;
use quick_xml::events::Event;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let body = dbg!(
        ureq::get("https://matklad.github.io/feed.xml")
            .call()?
            .body_mut()
            .read_to_string()?
    );

    let feed = Feed::from(body.as_str());

    match feed {
        Feed::Rss(val) => {
            let channel = rss::Channel::read_from(val.as_bytes())?;
            println!("{channel:?}");
        }
        Feed::Atom(val) => {
            let channel = atom_syndication::Feed::read_from(val.as_bytes())?;
            println!("{channel:?}");
        }
        Feed::None => todo!(),
    }

    Ok(())
}

#[derive(Debug)]
pub enum Feed {
    Rss(String),
    Atom(String),
    None,
}
impl From<&str> for Feed {
    fn from(value: &str) -> Self {
        let mut reader = Reader::from_reader(value.as_bytes());
        let mut buf = Vec::new();
        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            // NOTE: this is the generic case when we don't know about the input BufRead.
            // when the input is a &str or a &[u8], we don't actually need to use another
            // buffer, we could directly call `reader.read_event()`
            match reader.read_event_into(&mut buf) {
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,
                Ok(Event::Start(tag)) => {
                    return match tag.local_name().as_ref() {
                        b"rss" | b"rdf" => Feed::Rss(value.to_string()),
                        b"feed" => Feed::Atom(value.to_string()),
                        _ => Feed::None,
                    };
                }
                _ => (),
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        todo!()
    }
}
