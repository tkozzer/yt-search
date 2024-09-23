use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
