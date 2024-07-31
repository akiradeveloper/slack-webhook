use reqwest::{Client, IntoUrl, Url};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Webhook URL ({0})")]
    InvalidUrl(reqwest::Error),
    #[error("Failed to send message ({0})")]
    SendError(reqwest::Error),
}

pub struct SlackWebhook {
    url: Option<Url>,
    prefix: Option<String>,
}
impl SlackWebhook {
    pub fn new(url: impl IntoUrl) -> Result<Self, Error> {
        let url = url.into_url().map_err(|e| Error::InvalidUrl(e))?;
        Ok(Self {
            url: Some(url),
            prefix: None,
        })
    }

    /// Make a fake client that actually does nothing.
    pub fn fake() -> Self {
        Self {
            url: None,
            prefix: None,
        }
    }

    /// Add prefix to every message.
    /// Between the message contents, a space is inserted.
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Send text to the webhook. Simple API.
    pub async fn send_text(&self, text: impl Into<String>) -> Result<serde_json::Value, Error> {
        let client = Client::new();
        let message = {
            let prefix = self
                .prefix
                .as_ref()
                .map(|x| format!("{} ", x))
                .unwrap_or_default();
            json!({
                "text": format!("{prefix}{}", text.into()),
            })
        };
        if let Some(url) = &self.url {
            let resp = client
                .post(url.clone())
                .json(&message)
                .send()
                .await
                .map_err(|e| Error::SendError(e))?;
            resp.error_for_status().map_err(|e| Error::SendError(e))?;
            Ok(message)
        } else {
            Ok(message)
        }
    }
}
