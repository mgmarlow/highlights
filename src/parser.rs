#[derive(Debug, Clone)]
pub enum Kind {
    Highlight,
    Note,
    Bookmark,
}

#[derive(Debug, Clone)]
pub struct Clipping {
    pub title: String,
    pub kind: Kind,
    pub page: Option<String>,
    pub location: String,
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

    let title = lines[0].trim().to_string();
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
        kind,
        page,
        location,
        date,
        content,
    })
}
