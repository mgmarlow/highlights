use crate::parser::{Clipping, Kind};
use rusqlite::{Connection, Result};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;

pub fn summary(clippings: &[Clipping]) {
    let book_counts = clippings.iter().fold(HashMap::new(), |mut acc, clipping| {
        *acc.entry(clipping.title.clone()).or_insert(0) += 1;
        acc
    });

    println!("Highlights by book:");
    for (title, count) in book_counts.iter() {
        println!("{}: {}", title, count);
    }

    let total: usize = book_counts.values().sum();
    println!("\nTotal: {}", total);
}

pub fn json(clippings: &[Clipping], outfile: Option<&str>) {
    match serde_json::to_string_pretty(clippings) {
        Ok(json) => {
            if let Some(path) = outfile {
                match File::create(path) {
                    Ok(mut file) => {
                        if let Err(e) = writeln!(file, "{}", json) {
                            eprintln!("Error writing to file: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Error creating file: {}", e),
                }
            } else {
                println!("{}", json);
            }
        }
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}

pub fn sqlite(clippings: &[Clipping], outfile: Option<&str>) {
    let db_path = outfile.unwrap_or("highlights.db");

    match create_database(db_path, clippings) {
        Ok((highlights_count, notes_count)) => {
            println!("Successfully created database at: {}", db_path);
            println!(
                "Inserted {} highlights and {} notes",
                highlights_count, notes_count
            );
        }
        Err(e) => eprintln!("Error creating database: {}", e),
    }
}

fn create_database(db_path: &str, clippings: &[Clipping]) -> Result<(usize, usize)> {
    let conn = Connection::open(db_path)?;

    // Load and execute schema
    let schema = fs::read_to_string("config/schema.sql").expect("Failed to read schema.sql");
    conn.execute_batch(&schema)?;

    // Partition clippings by kind
    let (highlights, notes): (Vec<_>, Vec<_>) = clippings
        .iter()
        .partition(|c| matches!(c.kind, Kind::Highlight | Kind::Bookmark));

    // Insert all highlights first
    let mut highlights_count = 0;
    for highlight in highlights {
        conn.execute(
            "INSERT INTO highlights (title, page, location, date, content) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &highlight.title,
                &highlight.page,
                &highlight.location,
                &highlight.date,
                &highlight.content,
            ),
        )?;
        highlights_count += 1;
    }

    // Insert all notes, linking to the most recent highlight from the same book
    let mut notes_count = 0;
    for note in notes {
        // Find the most recent highlight from the same book
        let mut stmt =
            conn.prepare("SELECT id FROM highlights WHERE title = ?1 ORDER BY id DESC LIMIT 1")?;
        let mut rows = stmt.query([&note.title])?;

        if let Some(row) = rows.next()? {
            let highlight_id: i64 = row.get(0)?;
            conn.execute(
                "INSERT INTO notes (content, highlight_id) VALUES (?1, ?2)",
                (&note.content, highlight_id),
            )?;
            notes_count += 1;
        } else {
            eprintln!(
                "Warning: Could not find matching highlight for note in '{}'",
                note.title
            );
        }
    }

    Ok((highlights_count, notes_count))
}
