use crate::parser::Clipping;
use std::fs::File;
use std::io::Write;

pub fn summary(clippings: &[Clipping]) {
    println!("Found {} highlights", clippings.len());
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
