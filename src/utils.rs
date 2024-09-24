use crate::search::{Duration, SearchFilters, SortBy};

pub fn build_search_url(query: &str, filters: &SearchFilters) -> String {
    let mut url = format!("https://www.youtube.com/results?search_query={}", query);

    if let Some(sp_code) =
        generate_sp_parameter(filters.sort_by.as_ref(), filters.duration.as_ref())
    {
        url.push_str(&format!("&sp={}", sp_code));
    }

    url
}

fn generate_sp_parameter(
    sort_by: Option<&SortBy>,
    duration: Option<&Duration>,
) -> Option<&'static str> {
    // If both filters are None, return None to avoid adding the sp parameter
    if sort_by.is_none() && duration.is_none() {
        return None;
    }

    // Define the mapping as a static constant
    const SP_CODES: &[((&str, &str), &str)] = &[
        // Relevance (Note: Removed ("Relevance", "None") entry)
        (("Relevance", "Short"), "EgIYAQ=="),
        (("Relevance", "Medium"), "EgIYAw=="),
        (("Relevance", "Long"), "EgIYAg=="),
        // View Count
        (("View Count", "None"), "CAM%3D"),
        (("View Count", "Short"), "CAMSAhgB"),
        (("View Count", "Medium"), "CAMSAhgD"),
        (("View Count", "Long"), "CAMSAhgC"),
        // Upload Date
        (("Upload Date", "None"), "CAI%3D"),
        (("Upload Date", "Short"), "CAISAhgB"),
        (("Upload Date", "Medium"), "CAISAhgD"),
        (("Upload Date", "Long"), "CAISAhgC"),
        // Rating
        (("Rating", "None"), "CAESAhAB"),
        (("Rating", "Short"), "CAESAhgB"),
        (("Rating", "Medium"), "CAESAhgD"),
        (("Rating", "Long"), "CAESAhgC"),
    ];

    // Convert the options to strings for matching
    let sort_by_str = match sort_by {
        Some(SortBy::Relevance) | None => "Relevance",
        Some(SortBy::ViewCount) => "View Count",
        Some(SortBy::UploadDate) => "Upload Date",
        Some(SortBy::Rating) => "Rating",
    };

    let duration_str = match duration {
        Some(Duration::Short) => "Short",
        Some(Duration::Medium) => "Medium",
        Some(Duration::Long) => "Long",
        None => "None",
    };

    // If sort_by is Relevance and duration is None, return None to avoid adding sp parameter
    if sort_by_str == "Relevance" && duration_str == "None" {
        return None;
    }

    // Search for the matching sp_code
    for &((s_by, dur), code) in SP_CODES {
        if s_by == sort_by_str && dur == duration_str {
            return Some(code);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::{Duration, SortBy};

    #[test]
    fn test_relevance_none() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::Relevance),
                    duration: None
                }
            ),
            "https://www.youtube.com/results?search_query=test"
        );
    }

    #[test]
    fn test_relevance_short() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::Relevance),
                    duration: Some(Duration::Short)
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=EgIYAQ=="
        );
    }

    #[test]
    fn test_relevance_medium() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::Relevance),
                    duration: Some(Duration::Medium)
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=EgIYAw=="
        );
    }

    #[test]
    fn test_relevance_long() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::Relevance),
                    duration: Some(Duration::Long)
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=EgIYAg=="
        );
    }

    #[test]
    fn test_view_count_none() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::ViewCount),
                    duration: None
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=CAM%3D"
        );
    }

    #[test]
    fn test_view_count_short() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::ViewCount),
                    duration: Some(Duration::Short)
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=CAMSAhgB"
        );
    }

    #[test]
    fn test_view_count_medium() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::ViewCount),
                    duration: Some(Duration::Medium)
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=CAMSAhgD"
        );
    }

    #[test]
    fn test_view_count_long() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::ViewCount),
                    duration: Some(Duration::Long)
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=CAMSAhgC"
        );
    }

    #[test]
    fn test_upload_date_none() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::UploadDate),
                    duration: None
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=CAI%3D"
        );
    }

    #[test]
    fn test_upload_date_short() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::UploadDate),
                    duration: Some(Duration::Short)
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=CAISAhgB"
        );
    }

    #[test]
    fn test_upload_date_medium() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::UploadDate),
                    duration: Some(Duration::Medium)
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=CAISAhgD"
        );
    }

    #[test]
    fn test_upload_date_long() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::UploadDate),
                    duration: Some(Duration::Long)
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=CAISAhgC"
        );
    }

    #[test]
    fn test_rating_none() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::Rating),
                    duration: None
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=CAESAhAB"
        );
    }

    #[test]
    fn test_rating_short() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::Rating),
                    duration: Some(Duration::Short)
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=CAESAhgB"
        );
    }

    #[test]
    fn test_rating_medium() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::Rating),
                    duration: Some(Duration::Medium)
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=CAESAhgD"
        );
    }

    #[test]
    fn test_rating_long() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: Some(SortBy::Rating),
                    duration: Some(Duration::Long)
                }
            ),
            "https://www.youtube.com/results?search_query=test&sp=CAESAhgC"
        );
    }

    #[test]
    fn test_no_filters() {
        assert_eq!(
            build_search_url(
                "test",
                &SearchFilters {
                    sort_by: None,
                    duration: None
                }
            ),
            "https://www.youtube.com/results?search_query=test"
        );
    }
}
