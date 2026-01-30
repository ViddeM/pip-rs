use std::{collections::HashMap, net::IpAddr};

use common::IpResponse;
use futures::{StreamExt, stream::FuturesUnordered};
use reqwest::{Client, ClientBuilder, IntoUrl, StatusCode};
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum PingerError {
    #[error("Reqwest error: `{0}`")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Received an error response from remote with status {status} and body {body} ")]
    ErrorResponse { body: String, status: StatusCode },
    #[error("Received an invalid response body, error: {error}")]
    InvalidResponse { error: String },
}

pub struct IpPinger {
    client: Client,
    remotes: Vec<Url>,
}

impl IpPinger {
    fn new(remotes: Vec<Url>) -> Result<Self, PingerError> {
        let client = ClientBuilder::new().build()?;
        Ok(Self {
            client,
            remotes: remotes,
        })
    }

    const PING_ENDPOINT: &'static str = "/ip";
    pub async fn ping(&self) -> Result<IpAddr, HashMap<Url, PingerError>> {
        let mut futures = self
            .remotes
            .iter()
            .map(|remote| self.ping_remote(remote))
            .collect::<FuturesUnordered<_>>();

        let mut errors = HashMap::new();
        while let Some(resp) = futures.next().await {
            match resp {
                Ok(ip) => return Ok(ip),
                Err((remote, err)) => {
                    errors.insert(remote, err);
                }
            }
        }

        return Err(errors);
    }

    async fn ping_remote(&self, remote: &Url) -> Result<IpAddr, (Url, PingerError)> {
        match self.ping_remote_inner(remote).await {
            Ok(ip) => Ok(ip),
            Err(err) => Err((remote.clone(), err)),
        }
    }

    async fn ping_remote_inner(&self, remote: &Url) -> Result<IpAddr, PingerError> {
        let endpoint = format!("{remote}{}", Self::PING_ENDPOINT);
        let response = self.client.get(endpoint).send().await?;
        let response_status = response.status();
        let response_body = response.text().await?;
        if response_status != StatusCode::OK {
            return Err(PingerError::ErrorResponse {
                body: response_body,
                status: response_status,
            });
        }

        match IpResponse::parse(response_body) {
            Ok(resp) => Ok(resp.ip_addr()),
            Err(err) => Err(PingerError::InvalidResponse { error: err }),
        }
    }

    pub fn builder() -> IpPingerBuilder {
        IpPingerBuilder::new()
    }
}

pub struct IpPingerBuilder {
    remotes: Vec<Url>,
}

impl IpPingerBuilder {
    fn new() -> Self {
        Self { remotes: vec![] }
    }

    pub fn with_remote<U: IntoUrl>(mut self, remote: U) -> Result<Self, PingerError> {
        self.remotes.push(remote.into_url()?);
        Ok(self)
    }

    pub fn build(self) -> Result<IpPinger, PingerError> {
        IpPinger::new(self.remotes)
    }
}
