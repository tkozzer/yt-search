use crate::search::{Duration, SearchFilters, SortBy};
use base64::{engine::general_purpose, Engine as _};

pub fn build_search_url(query: &str, filters: &SearchFilters) -> String {
    let mut url = format!("https://www.youtube.com/results?search_query={}", query);

    let sp = generate_sp_parameter(filters.sort_by.as_ref(), filters.duration.as_ref());
    if !sp.is_empty() {
        url.push_str(&format!("&sp={}", sp));
    }

    url
}

fn generate_sp_parameter(sort_by: Option<&SortBy>, duration: Option<&Duration>) -> String {
    let mut sp = String::new();

    match sort_by {
        Some(SortBy::ViewCount) => sp.push_str("CAM"),
        Some(SortBy::UploadDate) => sp.push_str("CAI"),
        Some(SortBy::Rating) => sp.push_str("CAE"),
        _ => sp.push_str("CAA"), // Relevance or None
    }

    if let Some(duration) = duration {
        if sort_by.is_some() && sort_by.map(|s| *s != SortBy::Relevance).unwrap_or(false) {
            sp.push_str("S");
        }
        sp.push_str("AhgA"); // Base for duration
        match duration {
            Duration::Short => sp.push('Q'),
            Duration::Medium => sp.push('w'),
            Duration::Long => sp.push('g'),
        }
    }

    if !sp.is_empty() {
        general_purpose::STANDARD.encode(sp)
    } else {
        String::new()
    }
}
