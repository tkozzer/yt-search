use scraper::{Html, Selector};
use regex::Regex;
use serde_json::Value;
use crate::errors::ParserError;
use crate::models::VideoResult;

pub fn extract_initial_data(html: &str) -> Result<Value, ParserError> {
    let document = Html::parse_document(html);
    let script_selector = Selector::parse("script").map_err(|_| ParserError::ExtractInitialDataError)?;

    let yt_initial_data_script = document
        .select(&script_selector)
        .find(|element| element.inner_html().contains("var ytInitialData"))
        .map(|element| element.inner_html())
        .ok_or(ParserError::ExtractInitialDataError)?;

    let re = Regex::new(r"var ytInitialData\s*=\s*(\{.*?\});").map_err(|_| ParserError::ExtractInitialDataError)?;
    let json_str = re
        .captures(&yt_initial_data_script)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .ok_or(ParserError::ExtractInitialDataError)?;

    serde_json::from_str(json_str).map_err(ParserError::JsonError)
}

pub fn parse_search_results(data: &Value) -> Result<Vec<VideoResult>, ParserError> {
    let mut videos = Vec::new();

    if let Some(contents) = data["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]
        ["sectionListRenderer"]["contents"]
        .as_array()
    {
        for content in contents {
            if let Some(items) = content["itemSectionRenderer"]["contents"].as_array() {
                for item in items {
                    if let Some(video_renderer) = item["videoRenderer"].as_object() {
                        let video = VideoResult {
                            video_id: video_renderer
                                .get("videoId")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string(),
                            title: video_renderer
                                .get("title")
                                .and_then(|t| t["runs"].as_array())
                                .and_then(|runs| runs.get(0))
                                .and_then(|run| run["text"].as_str())
                                .unwrap_or_default()
                                .to_string(),
                            channel_name: video_renderer
                                .get("ownerText")
                                .and_then(|o| o["runs"].as_array())
                                .and_then(|runs| runs.get(0))
                                .and_then(|run| run["text"].as_str())
                                .unwrap_or_default()
                                .to_string(),
                            view_count: video_renderer
                                .get("viewCountText")
                                .and_then(|v| v["simpleText"].as_str())
                                .unwrap_or_default()
                                .to_string(),
                            published_time: video_renderer
                                .get("publishedTimeText")
                                .and_then(|p| p["simpleText"].as_str())
                                .unwrap_or_default()
                                .to_string(),
                            duration: video_renderer
                                .get("lengthText")
                                .and_then(|l| l["simpleText"].as_str())
                                .unwrap_or_default()
                                .to_string(),
                            thumbnail_url: video_renderer
                                .get("thumbnail")
                                .and_then(|t| t["thumbnails"].as_array())
                                .and_then(|thumbs| thumbs.last())
                                .and_then(|thumb| thumb["url"].as_str())
                                .unwrap_or_default()
                                .to_string(),
                            description_snippet: video_renderer
                                .get("detailedMetadataSnippets")
                                .and_then(|d| d.as_array())
                                .and_then(|snippets| snippets.get(0))
                                .and_then(|snippet| snippet["snippetText"]["runs"].as_array())
                                .map(|runs| {
                                    runs.iter()
                                        .filter_map(|run| run["text"].as_str())
                                        .collect::<String>()
                                })
                                .unwrap_or_default(),
                        };

                        videos.push(video);
                    }
                }
            }
        }
    }

    Ok(videos)
}