use anyhow::anyhow;
use futures_lite::StreamExt;
use smol::{channel::unbounded, lock::Semaphore};
use std::{fmt::Display, sync::Arc};

use url::Url;

/// The body of an HTTP content fetched from the URL
#[derive(Debug, Clone)]
pub struct HttpContent(String);

impl HttpContent {
    pub async fn fetch_one(url: Url, client: &surf::Client) -> anyhow::Result<Self> {
        let content = client
            .get(url)
            .recv_string()
            .await
            .map_err(|e| anyhow!(e))?;

        Ok(Self(content))
    }

    /// Fetch the contents of a list of URLs.
    ///
    /// * `limit`: the maximum concurrent requests to make.
    /// * `urls`: the list of URLs to fetch
    /// * `client`: the Surf client to use to fetch the URLs
    pub async fn fetch(
        limit: usize,
        urls: Vec<Url>,
        client: &surf::Client,
    ) -> Vec<anyhow::Result<Self>> {
        let sem = Arc::new(Semaphore::new(limit));
        let (tx, rx) = unbounded::<anyhow::Result<Self>>();
        for url in urls {
            let tx = tx.clone();
            let client = client.clone();
            let sem = sem.clone();
            let permit = sem.acquire_arc().await;
            // Resolve each URL
            smol::spawn(async move {
                let res = HttpContent::fetch_one(url, &client);
                let _ = tx.send(res.await).await;
                // We're done; let others start fetching.
                drop(permit);
            })
            .detach();
        }
        drop(tx);
        rx.collect::<Vec<_>>().await
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
