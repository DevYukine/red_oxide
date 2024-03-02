use std::str;
use std::time::Duration;

use anyhow::Error;
use reqwest::{Client, Method, Response, Url};
use serde::de::DeserializeOwned;
use tower::limit::RateLimit;
use tower::{Service, ServiceExt};

use crate::redacted::api::constants::API_URL;
use crate::redacted::api::error::RedactedApiError;
use crate::redacted::api::model::{
    ApiResponse, ApiResponseReceived, IndexResponse, TorrentGroupResponse, TorrentResponse,
    TorrentUploadResponse,
};
use crate::redacted::upload::TorrentUploadData;
use crate::util::http::CLIENT;
use crate::util::http::USER_AGENT;

pub struct RedactedApi {
    client: Client,
    service: RateLimit<Client>,
    headers: reqwest::header::HeaderMap,
}

impl RedactedApi {
    pub fn new(api_key: String) -> anyhow::Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("User-Agent", USER_AGENT.parse()?);
        headers.insert("Accept", "application/json".parse()?);
        headers.insert("Authorization", api_key.parse()?);

        let client = CLIENT.clone();

        let service = tower::ServiceBuilder::new()
            .rate_limit(10, Duration::from_secs(10))
            .service(client.clone());

        Ok(Self {
            client,
            service,
            headers,
        })
    }

    pub async fn index(&mut self) -> anyhow::Result<ApiResponse<IndexResponse>> {
        return self
            .do_request_parsed::<IndexResponse>(Method::GET, vec![("action", "index")])
            .await;
    }

    pub async fn get_torrent_info(
        &mut self,
        torrent_id: i64,
    ) -> anyhow::Result<ApiResponse<TorrentResponse>> {
        return self
            .do_request_parsed::<TorrentResponse>(
                Method::GET,
                vec![
                    ("action", "torrent"),
                    ("id", torrent_id.to_string().as_str()),
                ],
            )
            .await;
    }

    pub async fn get_torrent_group(
        &mut self,
        group_id: i64,
    ) -> anyhow::Result<ApiResponse<TorrentGroupResponse>> {
        return self
            .do_request_parsed::<TorrentGroupResponse>(
                Method::GET,
                vec![
                    ("action", "torrentgroup"),
                    ("id", group_id.to_string().as_str()),
                ],
            )
            .await;
    }

    pub async fn download_torrent(&mut self, torrent_id: i64) -> anyhow::Result<Vec<u8>> {
        let req = self
            .client
            .request(Method::GET, Url::parse(API_URL)?)
            .query(&vec![
                ("action", "download"),
                ("id", torrent_id.to_string().as_str()),
            ])
            .headers(self.headers.clone())
            .build()?;

        let res = self.service.ready().await?.call(req).await?;

        if !res.status().is_success() {
            return Err(Error::from(RedactedApiError::DownloadError(
                str::from_utf8(&*res.bytes().await?)?.to_string(),
            )));
        }

        Ok(res.bytes().await?.to_vec())
    }

    pub async fn upload_torrent(
        &mut self,
        upload_data: TorrentUploadData,
    ) -> anyhow::Result<ApiResponse<TorrentUploadResponse>> {
        let form = upload_data.into();

        let req = self
            .client
            .request(Method::POST, Url::parse(API_URL)?)
            .query(&vec![("action", "upload")])
            .multipart(form)
            .headers(self.headers.clone())
            .build()?;

        let res = self.service.ready().await?.call(req).await?;

        if !res.status().is_success() {
            return Err(Error::from(RedactedApiError::UploadError(
                str::from_utf8(&*res.bytes().await?)?.to_string(),
            )));
        }

        Ok(self
            .handle_status_and_parse_body::<TorrentUploadResponse>(res)
            .await?)
    }

    async fn do_request_parsed<T: DeserializeOwned>(
        &mut self,
        method: Method,
        query: Vec<(&str, &str)>,
    ) -> anyhow::Result<ApiResponse<T>> {
        let req = self
            .client
            .request(method, Url::parse(API_URL)?)
            .query(&query)
            .headers(self.headers.clone())
            .build()?;

        let res = self.service.ready().await?.call(req).await?;

        Ok(self.handle_status_and_parse_body::<T>(res).await?)
    }

    async fn handle_status_and_parse_body<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> anyhow::Result<ApiResponse<T>> {
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
            Err(Error::from(RedactedApiError::NoSuccessStatusCodeError(
                status,
                parsed.error.unwrap_or("No Error Provided".to_string()),
            )))
        };
    }
}
