use std::io::{BufRead, BufReader, Result};
use std::{fs::File, path::PathBuf};


use chrono::{DateTime, Utc};

use crate::util::definitions;

pub fn get_file_object_from_path(filepath: PathBuf) -> Result<definitions::Blog> {
    let input = File::open(filepath)?;
    let modified_date: DateTime<Utc> = input.metadata()?.modified()?.into();
    let mut reader = BufReader::new(input);
    let mut head_line = String::new();
    let _ = reader.read_line(&mut head_line)?;
    //get Head Title and the reset buffer
    let title = head_line[2..head_line.len()].trim();
    let mut preview_line = String::new();
    let _ = reader.read_line(&mut preview_line)?;
    let preview = preview_line.trim();
    let blog = definitions::Blog {
        title: title.to_string(),
        content: preview.to_string(),
        date: modified_date.to_string(),
    };
    Ok(blog)
}