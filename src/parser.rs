#[derive(Debug, Clone, serde::Serialize)]
pub enum Kind {
    Highlight,
    Note,
    Bookmark,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Clipping {
    pub title: String,
    pub author: Option<String>,
    pub kind: Kind,
    pub page: Option<String>,
    pub location: String,
    pub location_start: i64,
    pub location_end: Option<i64>,
    pub date: String,
    pub content: String,
}

pub fn parse(contents: &str) -> Vec<Clipping> {
    contents
        .split("==========")
        .filter_map(parse_clipping)
        .collect()
}

fn parse_clipping(entry: &str) -> Option<Clipping> {
    let entry = entry.trim();
    if entry.is_empty() {
        return None;
    }

    let lines: Vec<&str> = entry.lines().collect();
    if lines.len() < 2 {
        return None;
    }

    let title_line = lines[0].trim();
    let (title, author) = parse_title_and_author(title_line);
    let metadata = lines[1].trim();

    // Parse metadata line
    // Format: "- Your [TYPE] on page X | Location Y | Added on [DATE]"
    let kind = if metadata.contains("Highlight") {
        Kind::Highlight
    } else if metadata.contains("Note") {
        Kind::Note
    } else if metadata.contains("Bookmark") {
        Kind::Bookmark
    } else {
        return None; // Skip unknown types
    };

    let page = if let Some(page_start) = metadata.find("page ") {
        if let Some(page_end) = metadata[page_start..].find(" |") {
            Some(metadata[page_start + 5..page_start + page_end].to_string())
        } else {
            None
        }
    } else {
        None
    };

    let location = if let Some(loc_start) = metadata.find("Location ") {
        if let Some(loc_end) = metadata[loc_start..].find(" |") {
            metadata[loc_start + 9..loc_start + loc_end].to_string()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };

    // Parse location into start and end integers
    let (location_start, location_end) = parse_location_range(&location)?;

    let date = if let Some(date_start) = metadata.find("Added on ") {
        metadata[date_start + 9..].to_string()
    } else {
        "".to_string()
    };

    // Content is everything after the metadata line (skipping blank line)
    let content = if lines.len() > 3 {
        lines[3..].join("\n").trim().to_string()
    } else {
        "".to_string()
    };

    Some(Clipping {
        title,
        author,
        kind,
        page,
        location,
        location_start,
        location_end,
        date,
        content,
    })
}

fn parse_location_range(location: &str) -> Option<(i64, Option<i64>)> {
    if location.is_empty() {
        return None;
    }

    if let Some(dash_pos) = location.find('-') {
        // Range format: "2898-2899"
        let start = location[..dash_pos].parse().ok()?;
        let end = location[dash_pos + 1..].parse().ok()?;
        Some((start, Some(end)))
    } else {
        // Single number: "2829"
        let num = location.parse().ok()?;
        Some((num, None))
    }
}

fn parse_title_and_author(title_line: &str) -> (String, Option<String>) {
    // Check if title contains author in parentheses at the end
    // Format: "Book Title (Author Name)"
    if let Some(open_paren) = title_line.rfind('(') {
        if title_line.ends_with(')') {
            let title = title_line[..open_paren].trim().to_string();
            let author = title_line[open_paren + 1..title_line.len() - 1]
                .trim()
                .to_string();
            return (title, Some(author));
        }
    }
    (title_line.to_string(), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_highlight_with_page() {
        let input = "Hackers & Painters
- Your Highlight on page 216 | Location 2898-2899 | Added on Monday, March 4, 2024 5:17:31 PM

So the short explanation of why this 1950s language is not obsolete is that it was not technology but math, and math doesn't get stale.";

        let clipping = parse_clipping(input).unwrap();

        assert_eq!(clipping.title, "Hackers & Painters");
        assert_eq!(clipping.author, None);
        assert!(matches!(clipping.kind, Kind::Highlight));
        assert_eq!(clipping.page, Some("216".to_string()));
        assert_eq!(clipping.location, "2898-2899");
        assert_eq!(clipping.location_start, 2898);
        assert_eq!(clipping.location_end, Some(2899));
        assert_eq!(clipping.date, "Monday, March 4, 2024 5:17:31 PM");
        assert_eq!(
            clipping.content,
            "So the short explanation of why this 1950s language is not obsolete is that it was not technology but math, and math doesn't get stale."
        );
    }

    #[test]
    fn test_parse_note() {
        let input = "Coders at Work (Seibel, Peter)
- Your Note on page 146 | Location 2829 | Added on Monday, March 18, 2024 10:28:48 PM

Well said";

        let clipping = parse_clipping(input).unwrap();

        assert_eq!(clipping.title, "Coders at Work");
        assert_eq!(clipping.author, Some("Seibel, Peter".to_string()));
        assert!(matches!(clipping.kind, Kind::Note));
        assert_eq!(clipping.page, Some("146".to_string()));
        assert_eq!(clipping.location, "2829");
        assert_eq!(clipping.location_start, 2829);
        assert_eq!(clipping.location_end, None);
        assert_eq!(clipping.content, "Well said");
    }

    #[test]
    fn test_parse_bookmark() {
        let input = "Coders at Work (Seibel, Peter)
- Your Bookmark on page 160 | Location 3085 | Added on Tuesday, March 19, 2024 10:35:54 PM

";

        let clipping = parse_clipping(input).unwrap();

        assert_eq!(clipping.title, "Coders at Work");
        assert_eq!(clipping.author, Some("Seibel, Peter".to_string()));
        assert!(matches!(clipping.kind, Kind::Bookmark));
        assert_eq!(clipping.page, Some("160".to_string()));
        assert_eq!(clipping.location, "3085");
        assert_eq!(clipping.location_start, 3085);
        assert_eq!(clipping.location_end, None);
        assert_eq!(clipping.content, "");
    }

    #[test]
    fn test_parse_empty_input() {
        let result = parse_clipping("");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_invalid_format() {
        let input = "Just a title line
Not a valid metadata line";

        let result = parse_clipping(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_multiple_clippings() {
        let input = "Hackers & Painters
- Your Highlight on page 216 | Location 2898-2899 | Added on Monday, March 4, 2024 5:17:31 PM

First highlight content
==========
Coders at Work (Seibel, Peter)
- Your Note on page 146 | Location 2829 | Added on Monday, March 18, 2024 10:28:48 PM

Note content
==========";

        let clippings = parse(input);

        assert_eq!(clippings.len(), 2);
        assert_eq!(clippings[0].title, "Hackers & Painters");
        assert_eq!(clippings[0].author, None);
        assert!(matches!(clippings[0].kind, Kind::Highlight));
        assert_eq!(clippings[1].title, "Coders at Work");
        assert_eq!(clippings[1].author, Some("Seibel, Peter".to_string()));
        assert!(matches!(clippings[1].kind, Kind::Note));
    }

    #[test]
    fn test_parse_multiline_content() {
        let input = "Test Book
- Your Highlight on page 1 | Location 1-5 | Added on Monday, January 1, 2024 12:00:00 PM

This is a multiline
highlight that spans
multiple lines.";

        let clipping = parse_clipping(input).unwrap();

        assert_eq!(
            clipping.content,
            "This is a multiline\nhighlight that spans\nmultiple lines."
        );
    }
}
