use std::io::{BufRead, BufReader, Result, Seek};
use std::{fs::File};


use chrono::{DateTime, Utc};
use safe_path::scoped_join;
use crate::util::definitions;

pub fn get_file_object_from_path(filepath: &str) -> Result<definitions::BlogPreview> {
    let input = File::open(scoped_join(definitions::FILEPATH, filepath)?)?;
    let modified_date: DateTime<Utc> = get_modified_from_file(&input)?;
    let mut reader = BufReader::new(input);
    let (title,preview) = get_title_and_preview_from_reader(&mut reader)?;
    let blog = definitions::BlogPreview {
        title: title.to_string(),
        preview: preview.to_string(),
        date: modified_date.to_string(),
    };
    Ok(blog)
}
pub fn get_modified_from_file(file: &File) -> Result<DateTime<Utc>>{
    Ok(file.metadata()?.modified()?.into())
}
pub fn get_title_from_reader(reader: &mut BufReader<File>) -> Result<String>{
    let mut head_line = String::new();
    let _ = reader.read_line(&mut head_line)?;
    //get Head Title and the reset buffer
    let title: String = head_line[2..head_line.len()].trim().into();
    Ok(title)
}
pub fn get_title_and_preview_from_reader(reader: &mut BufReader<File>) -> Result<(String,String)>{
    let mut head_line = String::new();
    let _ = reader.read_line(&mut head_line)?;
    let title: String = head_line[2..head_line.len()].trim().into();
    let mut preview_line = String::new();
    let _ = reader.read_line(&mut preview_line)?;
    let preview:String = preview_line.trim().into();
    Ok((title,preview))
}