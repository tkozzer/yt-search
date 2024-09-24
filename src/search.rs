use crate::errors::SearchError;
use crate::models::VideoResult;
use crate::parser::{extract_initial_data, parse_search_results};
use crate::utils::build_search_url;
use log::error;
use reqwest::Client;
use serde_json::Value;
use std::error::Error as StdError;

pub struct YouTubeSearch {
    client: Client,
    #[allow(dead_code)]
    proxy: Option<String>,
    #[allow(dead_code)]
    debug: bool,
}

#[derive(Debug, Clone)]
pub struct SearchFilters {
    pub sort_by: Option<SortBy>,
    pub duration: Option<Duration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    Relevance,
    ViewCount,
    UploadDate,
    Rating,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Duration {
    Short,
    Medium,
    Long,
}

impl YouTubeSearch {
    pub fn new(proxy: Option<String>, debug: bool) -> Result<Self, Box<dyn StdError>> {
        let mut client_builder = Client::builder();
        if let Some(proxy_url) = &proxy {
            let proxy = reqwest::Proxy::all(proxy_url).map_err(|e| {
                error!("Failed to create proxy: {}", e);
                e
            })?;
            client_builder = client_builder.proxy(proxy);
        }

        let client = client_builder.build().map_err(|e| {
            error!("Failed to build client: {}", e);
            e
        })?;

        Ok(YouTubeSearch {
            client,
            proxy,
            debug,
        })
    }

    pub async fn search(
        &self,
        query: &str,
        filters: SearchFilters,
    ) -> Result<Vec<VideoResult>, SearchError> {
        let url = build_search_url(query, &filters);
        log::info!("Performing search for query: {}", query);
        log::info!("Applied filters: {:?}", filters);
        log::debug!("Using URL: {}", url);

        let html = self.fetch_search_page(&url).await?;
        let initial_data = extract_initial_data(&html)?;
        let results = parse_search_results(&initial_data)?;

        log::info!("Found {} results", results.len());
        Ok(results)
    }

    async fn fetch_search_page(&self, url: &str) -> Result<String, SearchError> {
        let response = self.client.get(url).send().await?;

        if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            log::warn!("Rate limit exceeded");
            return Err(SearchError::RateLimitExceeded);
        }

        let html = response.text().await?;
        Ok(html)
    }

    pub async fn check_ip(&self) -> Result<String, SearchError> {
        let ip_check_url = "https://api.ipify.org?format=json";
        let ip_response = self
            .client
            .get(ip_check_url)
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(ip_response["ip"].as_str().unwrap_or("Unknown").to_string())
    }
}
