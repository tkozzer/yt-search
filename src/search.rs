use reqwest::Client;
use crate::errors::SearchError;
use crate::models::VideoResult;
use crate::parser::{extract_initial_data, parse_search_results};
use crate::utils::build_search_url;

pub struct YouTubeSearch {
    client: Client,
    proxy: Option<String>,
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
    pub fn new(proxy: Option<String>, debug: bool) -> Self {
        let mut client_builder = Client::builder();
        if let Some(proxy_url) = &proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::all(proxy_url).unwrap());
        }
        let client = client_builder.build().unwrap();

        if debug {
            env_logger::init();
            log::set_max_level(log::LevelFilter::Debug);
        } else {
            log::set_max_level(log::LevelFilter::Info);
        }

        YouTubeSearch {
            client,
            proxy,
            debug,
        }
    }

    pub async fn search(&self, query: &str, filters: SearchFilters) -> Result<Vec<VideoResult>, SearchError> {
        let url = build_search_url(query, &filters);
        log::info!("Performing search for query: {}", query);
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
}