use smol::fs::read_to_string;
use url::Url;

/// The body of an HTTP content fetched from the URL
pub struct HttpContent(String);

impl TryFrom<Url> for HttpContent {
    type Error = anyhow::Error;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let content = ureq::agent()
            .get(value.as_str())
            .call()?
            .body_mut()
            .read_to_string()?;

        Ok(Self(content))
    }
}
