use crate::parser::Clipping;
use std::collections::HashMap;
use std::fs::File;
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
