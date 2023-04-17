use std::str;
use std::time::Duration;

use anyhow::Error;
use reqwest::{Client, Method, Request, Response, Url};
use serde::de::DeserializeOwned;
use tower::limit::RateLimit;
use tower::{Service, ServiceExt};

use crate::built_info;
use crate::redacted::api::constants::API_URL;
use crate::redacted::api::error::RedactedApiError;
use crate::redacted::api::model::{
    ApiResponse, ApiResponseReceived, IndexResponse, TorrentGroupResponse, TorrentResponse,
    TorrentUploadResponse,
};
use crate::redacted::upload::TorrentUploadData;

pub struct RedactedApi {
    client: Client,
    service: RateLimit<Client>,
}

impl RedactedApi {
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "User-Agent",
                    format!("{}/{} ", built_info::PKG_NAME, built_info::PKG_VERSION)
                        .parse()
                        .unwrap(),
                );
                headers.insert("Accept", "application/json".parse().unwrap());
                headers.insert("Authorization", api_key.parse().unwrap());
                headers
            })
            .build()
            .unwrap();

        let service = tower::ServiceBuilder::new()
            .rate_limit(10, Duration::from_secs(10))
            .service(client.clone());

        Self { client, service }
    }

    pub async fn index(&mut self) -> anyhow::Result<ApiResponse<IndexResponse>> {
        let req = Request::new(Method::POST, Url::parse(&(API_URL.to_owned() + "index"))?);

        let res = self.service.ready().await?.call(req).await?;

        Ok(self
            .handle_status_and_parse_body::<IndexResponse>(res)
            .await?)
    }

    pub async fn get_torrent_info(
        &mut self,
        torrent_id: i64,
    ) -> anyhow::Result<ApiResponse<TorrentResponse>> {
        let req = Request::new(
            Method::GET,
            Url::parse(&(API_URL.to_owned() + &format!("torrent&id={}", torrent_id.to_string())))?,
        );

        let res = self.service.ready().await?.call(req).await?;

        Ok(self
            .handle_status_and_parse_body::<TorrentResponse>(res)
            .await?)
    }

    pub async fn get_torrent_group(
        &mut self,
        group_id: i64,
    ) -> anyhow::Result<ApiResponse<TorrentGroupResponse>> {
        let req = Request::new(
            Method::GET,
            Url::parse(
                &(API_URL.to_owned() + &format!("torrentgroup&id={}", group_id.to_string())),
            )?,
        );

        let res = self.service.ready().await?.call(req).await?;

        Ok(self
            .handle_status_and_parse_body::<TorrentGroupResponse>(res)
            .await?)
    }

    pub async fn download_torrent(&mut self, torrent_id: i64) -> anyhow::Result<Vec<u8>> {
        let req = Request::new(
            Method::GET,
            Url::parse(&(API_URL.to_owned() + &format!("download&id={}", torrent_id.to_string())))?,
        );

        let res = self.service.ready().await?.call(req).await?;

        Ok(res.bytes().await?.to_vec())
    }

    pub async fn upload_torrent(
        &mut self,
        upload_data: TorrentUploadData,
    ) -> anyhow::Result<ApiResponse<TorrentUploadResponse>> {
        let form = upload_data.into();

        let req = self
            .client
            .request(Method::POST, Url::parse(&(API_URL.to_owned() + "upload"))?)
            .multipart(form)
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
            Err(Error::from(RedactedApiError::AuthError(
                parsed.error.unwrap_or("No Error Provided".to_string()),
            )))
        };
    }
}
