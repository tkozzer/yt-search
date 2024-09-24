# YouTube Search Scraping Library

A Rust library for scraping YouTube search results without using the official API.

## Features

- Perform YouTube searches programmatically
- Extract detailed information about video results
- Support for search filters (sort by, duration)
- **Enhanced privacy and rate limit avoidance with Tor/SOCKS proxy support**
- Asynchronous design compatible with Tauri V2
- Configurable logging for easy debugging

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
yt-search = "0.1.0"
```

## Tor/SOCKS Proxy Setup (Recommended)

For enhanced privacy and to avoid rate limits, we strongly recommend using a Tor SOCKS proxy.

### macOS Users

We've provided a convenient setup script for macOS users:

1. Clone this repository or download the `setup.sh` script.
2. Make the script executable:
   ```
   chmod +x setup.sh
   ```
3. Run the setup script:
   ```
   ./setup.sh
   ```

This script will automatically:
- Install Homebrew (if not already installed)
- Install Tor using Homebrew
- Start the Tor service
- Set up the SOCKS proxy on `127.0.0.1:9050`

After running the script, you'll be ready to use the Tor SOCKS proxy with this library.

### Other Operating Systems

For other operating systems, please refer to the [Tor Project's documentation](https://2019.www.torproject.org/docs/documentation.html.en) for installation instructions.

## Usage

Here's a basic example of how to use the library with the Tor SOCKS proxy:

```rust
use yt_search::{YouTubeSearch, SearchFilters, SortBy, Duration};

#[tokio::main]
async fn main() {
    // Use the Tor SOCKS proxy
    let search = YouTubeSearch::new(Some("socks5://127.0.0.1:9050"), false);
    let filters = SearchFilters {
        sort_by: Some(SortBy::ViewCount),
        duration: Some(Duration::Long),
    };
    
    match search.search("rust programming", filters).await {
        Ok(results) => {
            for result in results {
                println!("Title: {}", result.title);
                println!("Channel: {}", result.channel_name);
                println!("Views: {}", result.view_count);
                println!("---");
            }
        },
        Err(e) => eprintln!("Search error: {}", e),
    }
}
```

## API Reference

### `YouTubeSearch`

The main struct for performing searches.

#### Methods

- `new(proxy: Option<String>, debug: bool) -> Self`: Create a new `YouTubeSearch` instance.
- `search(&self, query: &str, filters: SearchFilters) -> Result<Vec<VideoResult>, SearchError>`: Perform a search with the given query and filters.
- `check_ip(&self) -> Result<String, SearchError>`: Check the IP address being used for requests.

### `SearchFilters`

A struct to specify search filters.

```rust
pub struct SearchFilters {
    pub sort_by: Option<SortBy>,
    pub duration: Option<Duration>,
}
```

### `SortBy`

An enum to specify the sort order of results.

```rust
pub enum SortBy {
    Relevance,
    ViewCount,
    UploadDate,
    Rating,
}
```

### `Duration`

An enum to specify the duration filter for videos.

```rust
pub enum Duration {
    Short,
    Medium,
    Long,
}
```

### `VideoResult`

A struct containing information about a single video result.

```rust
pub struct VideoResult {
    pub video_id: String,
    pub title: String,
    pub channel_name: String,
    pub view_count: String,
    pub published_time: String,
    pub duration: String,
    pub thumbnail_url: String,
    pub description_snippet: String,
}
```

## Error Handling

The library uses custom error types for better error handling:

- `SearchError`: Encompasses all errors that can occur during a search operation.
- `ParserError`: Specific errors related to parsing search results.

## Logging

The library uses the `log` crate for logging. You can configure logging levels and output destinations using `log4rs` or any other logging implementation compatible with the `log` crate.

## Proxy Support

We strongly recommend using a SOCKS proxy, particularly Tor, for enhanced privacy and to avoid rate limits. You can specify a proxy when creating a `YouTubeSearch` instance:

```rust
// Using the Tor SOCKS proxy (after running setup.sh)
let search = YouTubeSearch::new(Some("socks5://127.0.0.1:9050"), true);

// Using a custom SOCKS proxy
let search = YouTubeSearch::new(Some("socks5://your_proxy_address:port"), true);
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.