# YouTube Search Scraping Library Design Document

## 1. Library Structure

```
src/
├── lib.rs
├── search.rs
├── parser.rs
├── models.rs
├── errors.rs
└── utils.rs
```

## 2. Main Components

### 2.1 Search Module (`search.rs`)

This module will handle the main search functionality.

```rust
pub struct YouTubeSearch {
    client: reqwest::Client,
    proxy: Option<String>,
}

impl YouTubeSearch {
    pub fn new(proxy: Option<String>) -> Self { ... }

    pub async fn search(&self, query: &str, filters: SearchFilters) -> Result<Vec<VideoResult>, SearchError> { ... }

    async fn fetch_search_page(&self, url: &str) -> Result<String, SearchError> { ... }
}

pub struct SearchFilters {
    pub sort_by: Option<SortBy>,
    pub duration: Option<Duration>,
}

pub enum SortBy {
    Relevance,
    UploadDate,
    ViewCount,
    Rating,
}

pub enum Duration {
    Short,
    Medium,
    Long,
}
```

### 2.2 Parser Module (`parser.rs`)

This module will handle parsing the HTML and extracting the `ytInitialData`.

```rust
pub fn extract_initial_data(html: &str) -> Result<serde_json::Value, ParserError> { ... }

pub fn parse_search_results(data: &serde_json::Value) -> Result<Vec<VideoResult>, ParserError> { ... }
```

### 2.3 Models Module (`models.rs`)

This module will define the data structures for the search results.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoResult {
    pub video_id: String,
    pub title: String,
    pub channel_name: String,
    pub view_count: u64,
    pub published_time: String,
    pub duration: String,
    pub thumbnail_url: String,
    pub description_snippet: String,
}
```

### 2.4 Errors Module (`errors.rs`)

This module will define custom error types for the library.

```rust
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Parsing error: {0}")]
    ParsingError(#[from] ParserError),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    // Add more error types as needed
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Failed to extract ytInitialData")]
    ExtractInitialDataError,

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    // Add more parsing-specific errors
}
```

### 2.5 Utils Module (`utils.rs`)

This module will contain utility functions, such as building the search URL.

```rust
pub fn build_search_url(query: &str, filters: &SearchFilters) -> String { ... }
```

## 3. External Dependencies

- `reqwest`: For making HTTP requests
- `scraper`: For parsing HTML
- `serde` and `serde_json`: For JSON parsing
- `thiserror`: For error handling
- `log`: For logging
- `env_logger`: For configurable logging
- `tokio`: For async runtime

## 4. Error Handling and Logging

- Use the `log` crate for logging throughout the library
- Implement custom error types using `thiserror`
- Propagate errors up the call stack, providing context where necessary
- Implement four log levels: INFO, DEBUG, WARN, and ERROR
- Use `env_logger` to allow runtime configuration of log levels
- Implement a debug flag to enable/disable debug logging:

```rust
pub struct YouTubeSearch {
    client: reqwest::Client,
    proxy: Option<String>,
    debug: bool,
}

impl YouTubeSearch {
    pub fn new(proxy: Option<String>, debug: bool) -> Self {
        // Initialize logger
        env_logger::init();
        // Set log level based on debug flag
        if debug {
            log::set_max_level(log::LevelFilter::Debug);
        } else {
            log::set_max_level(log::LevelFilter::Info);
        }
        // ... rest of the implementation
    }
    
    // ... other methods
}
```

Usage of logging in the library:

```rust
log::info!("Performing search for query: {}", query);
log::debug!("Using filters: {:?}", filters);
log::warn!("Rate limit threshold reached, consider implementing rate limiting");
log::error!("Failed to parse search results: {}", error);
```

## 5. Proxy Support

- Implement SOCKS proxy support using the `reqwest` client's proxy settings

## 6. Rate Limiting

- Implement a simple rate limiting mechanism (e.g., delay between requests)
- Consider using a more advanced rate limiting library in the future

## 7. Testing

- Implement unit tests for individual functions
- Create integration tests for the main search functionality
- Use mock responses for testing to avoid actual network requests

## 8. Documentation

- Use Rust doc comments (`///`) for public APIs
- Provide usage examples in the library's README

## 9. Tauri V2 Compatibility

- Ensure the library is built with async/await patterns compatible with Tauri V2
- Use `tokio` as the async runtime, which is compatible with Tauri V2
- Expose the library's functionality through async functions that can be easily called from Tauri V2 commands

Example of a Tauri V2 command using the library:

```rust
use youtube_search_lib::YouTubeSearch;

#[tauri::command]
async fn search_youtube(query: String, sort_by: Option<String>, duration: Option<String>) -> Result<Vec<VideoResult>, String> {
    let search = YouTubeSearch::new(None, false);
    let filters = SearchFilters {
        sort_by: sort_by.map(|s| s.parse().unwrap()),
        duration: duration.map(|d| d.parse().unwrap()),
    };
    search.search(&query, filters).await.map_err(|e| e.to_string())
}
```

## 10. Feature Roadmap

- Implement remaining search filters (UPLOAD DATE, TYPE, FEATURES)
- Add support for pagination of search results
- Implement caching mechanism to reduce API calls and improve performance
- Add support for additional data extraction (e.g., video descriptions, channel info)
- Implement more advanced rate limiting strategies
- Add support for concurrent searches to improve performance
- Implement retry mechanism for failed requests
- Add support for localization (different YouTube domains and languages)