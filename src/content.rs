use std::fmt::Display;

use url::Url;

/// The body of an HTTP content fetched from the URL
#[derive(Debug, Clone)]
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

impl From<HttpContent> for String {
    fn from(HttpContent(value): HttpContent) -> Self {
        value
    }
}

impl Display for HttpContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content: String = self.clone().into();
        write!(f, "{content}")
    }
}
