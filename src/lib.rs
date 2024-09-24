mod errors;
mod logging;
mod models;
mod parser;
mod search;
mod utils;

pub use errors::{ParserError, SearchError};
pub use logging::init_logger;
pub use models::VideoResult;
pub use search::{Duration, SearchFilters, SortBy, YouTubeSearch}; // Add this line
