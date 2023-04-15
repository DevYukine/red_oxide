use crate::api::error::RedactedApiError;
use crate::api::model::{ApiResponse, ApiResponseReceived, IndexResponse};
use crate::built_info;
use anyhow::Error;
use reqwest::Response;
use serde::de::DeserializeOwned;

const API_URL: &str = "https://redacted.ch/ajax.php?action=";
const TRACKER_URL: &str = "https://redacted.ch/torrents.php?action=download&id=";

pub struct RedactedApi {
    client: reqwest::Client,
}

impl RedactedApi {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "User-Agent",
                    format!("RedOxide/{}", built_info::PKG_VERSION)
                        .parse()
                        .unwrap(),
                );
                headers.insert("Accept", "application/json".parse().unwrap());
                headers.insert("Authorization", api_key.parse().unwrap());
                headers
            })
            .build()
            .unwrap();
        Self { client }
    }

    pub async fn index(&self) -> Result<ApiResponse<IndexResponse>, Error> {
        let res = self
            .client
            .post(API_URL.to_owned() + "index")
            .send()
            .await?;

        Ok(self
            .handle_status_and_parse_body::<IndexResponse>(res)
            .await?)
    }

    pub async fn get_torrent_info(&self, torrent_id: i64) -> Result<(), reqwest::Error> {
        let res = self
            .client
            .post(API_URL.to_owned() + &format!("torrent?id={}", torrent_id.to_string()))
            .send()
            .await?;

        //let text = res.json::<ApiResponse<>>().await?;
        Ok(())
    }

    async fn handle_status_and_parse_body<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<ApiResponse<T>, Error> {
        let status = response.status();

        let parsed: ApiResponseReceived<T> = response.json::<ApiResponseReceived<T>>().await?;

        return if status.is_success() && parsed.error.is_none() {
            let response = match parsed.response {
                None => return Err(Error::from(RedactedApiError::BodyError)),
                Some(r) => r,
            };

            Ok(ApiResponse {
                status: parsed.status,
                response,
            })
        } else {
            Err(Error::from(RedactedApiError::AuthError(
                parsed.error.unwrap_or("No Error Provided".to_string()),
            )))
        };
    }
}
