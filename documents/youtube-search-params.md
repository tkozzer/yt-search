# YouTube Search Filter Codes Analysis

## Overview

YouTube uses a parameter called `sp` in its search URLs to apply various filters. This document focuses on decoding the `sp` parameter for sort by and duration filters.

For practical examples of how to use these parameters, see [YouTube Search Parameter Examples](youtube-search-params-example.md).

## Structure of the `sp` Parameter

The `sp` parameter is a Base64-encoded string that represents different filter options. When decoded, it reveals a pattern of letters and numbers that correspond to specific filter choices.

## Sort By Filter Codes

The sort by filter is represented by the first few characters in the `sp` parameter:

- Relevance: `CAASAhAB` or not specified
- View Count: `CAM%253D` (URL-encoded, actual value: `CAM=`)
- Upload Date: `CAI%253D` (URL-encoded, actual value: `CAI=`)
- Rating: `CAESAhAB`

## Duration Filter Codes

Duration filters are represented by `EgIYX` where `X` is a letter:

- Under 4 minutes: `EgIYAQ%253D%253D` (URL-encoded, actual value: `EgIYAQ==`)
- 4 - 20 minutes: `EgIYAw%253D%253D` (URL-encoded, actual value: `EgIYAw==`)
- Over 20 minutes: `EgIYAg%253D%253D` (URL-encoded, actual value: `EgIYAg==`)

## Combining Sort By and Duration Filters

When combining sort by and duration filters, the structure changes slightly:

1. For Relevance (default sort):
   - The duration code is used as is.
   - Example (4 - 20 minutes): `sp=EgIYAw%253D%253D`

2. For other sort options:
   - The sort code comes first, followed by `S` and then the duration code.
   - Example (View Count, 4 - 20 minutes): `sp=CAMSAhgD`

## Encoding

Note that the `%253D` in the URLs is a double-encoded `=` sign. When actually used in a request, it should be single-encoded as `%3D`.

## Implementation Guide

To implement these filters in your Rust library:

1. Create enums for Sort By and Duration options:

```rust
pub enum SortBy {
    Relevance,
    ViewCount,
    UploadDate,
    Rating,
}

pub enum Duration {
    Short,  // Under 4 minutes
    Medium, // 4 - 20 minutes
    Long,   // Over 20 minutes
}
```

2. Implement a function to generate the `sp` parameter:

```rust
pub fn generate_sp_parameter(sort_by: Option<SortBy>, duration: Option<Duration>) -> String {
    let mut sp = String::new();
    
    match sort_by {
        Some(SortBy::ViewCount) => sp.push_str("CAM"),
        Some(SortBy::UploadDate) => sp.push_str("CAI"),
        Some(SortBy::Rating) => sp.push_str("CAE"),
        _ => sp.push_str("CAA"), // Relevance or None
    }
    
    if let Some(duration) = duration {
        if sort_by.is_some() && sort_by != Some(SortBy::Relevance) {
            sp.push_str("S");
        }
        sp.push_str("AhgA"); // Base for duration
        match duration {
            Duration::Short => sp.push('Q'),
            Duration::Medium => sp.push('w'),
            Duration::Long => sp.push('g'),
        }
    }
    
    // Base64 encode the string
    base64::encode(sp)
}
```

3. Use this function when building the YouTube search URL:

```rust
let sp = generate_sp_parameter(Some(SortBy::ViewCount), Some(Duration::Medium));
let url = format!("https://www.youtube.com/results?search_query={}&sp={}", query, sp);
```

This implementation will generate the correct `sp` parameter for the desired sort by and duration filters.