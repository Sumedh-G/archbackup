use crate::{Error, Package, Raur, SearchBy, AUR_RPC_URL};
use async_trait::async_trait;
use reqwest::Client;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "type")]
    response_type: String,
    error: Option<String>,
    results: Vec<Package>,
}

/// A handle for making AUR requests.
#[derive(Clone, Debug)]
pub struct Handle {
    client: Client,
    url: String,
}

#[async_trait]
impl Raur for Handle {
    type Err = Error;

    async fn raw_info<S: AsRef<str> + Send + Sync>(
        &self,
        pkg_names: &[S],
    ) -> Result<Vec<Package>, Error> {
        let mut params = pkg_names
            .iter()
            .map(|name| ("arg[]", name.as_ref()))
            .collect::<Vec<_>>();
        params.extend([("v", "5"), ("type", "info")]);

        self.request(&params).await
    }

    async fn search_by<S: AsRef<str> + Send + Sync>(
        &self,
        query: S,
        strategy: SearchBy,
    ) -> Result<Vec<Package>, Error> {
        self.request(&[
            ("v", "5"),
            ("type", "search"),
            ("by", &strategy.to_string()),
            ("arg", query.as_ref()),
        ])
        .await
    }
}

impl Default for Handle {
    fn default() -> Self {
        Handle {
            client: Client::new(),
            url: AUR_RPC_URL.to_string(),
        }
    }
}

impl Handle {
    /// Create a new handle with default settings.
    pub fn new() -> Self {
        Handle {
            client: Client::new(),
            url: AUR_RPC_URL.to_string(),
        }
    }

    /// Create a new handle with a given reqwest client.
    pub fn new_with_client(client: Client) -> Self {
        Handle {
            client,
            url: AUR_RPC_URL.to_string(),
        }
    }

    /// Create a new handle with a given url.
    pub fn new_with_url<S: Into<String>>(url: S) -> Self {
        Handle {
            client: Client::new(),
            url: url.into(),
        }
    }

    /// Create a new handle with a given reqwest client and url.
    pub fn new_with_settings<S: Into<String>>(client: Client, url: S) -> Self {
        Handle {
            client,
            url: url.into(),
        }
    }

    /// Internal URL of this Handle. This just points to AUR RPC URL if you did not explicitly
    /// set it.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Getter for this handle's reqwest client.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// A helper function for making a request with given parameters.
    async fn request(&self, params: &[(&str, &str)]) -> Result<Vec<Package>, Error> {
        let response = self.client.post(&self.url).form(params).send().await?;
        response.error_for_status_ref()?;
        let response: Response = response.json().await?;

        if response.response_type == "error" {
            Err(Error::Aur(
                response
                    .error
                    .unwrap_or_else(|| "No error message provided".to_string()),
            ))
        } else {
            Ok(response.results)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search() {
        let handle = Handle::default();

        let query = handle.search("zzzzzzz").await.unwrap();
        assert_eq!(0, query.len());

        let query = handle.search("spotify").await.unwrap();
        assert!(!query.is_empty());
    }

    #[tokio::test]
    async fn test_info() {
        let handle = Handle::default();

        let query = handle.info(&["pacman-git"]).await.unwrap();
        assert_eq!(query[0].name, "pacman-git");

        // I maintain these two packages, so I can verify they exist.
        let query = handle.info(&["screens", "screens-git"]).await;
        assert!(query.is_ok());

        let query = query.unwrap();
        assert_eq!(2, query.len());
    }
}
