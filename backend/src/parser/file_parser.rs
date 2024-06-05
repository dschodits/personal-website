use std::io::{BufRead, BufReader, Read, Result};
use std::fs::File;
use std::path::Path;
use chrono::{DateTime, Utc};
use safe_path::scoped_join;
use crate::util::definitions;
pub fn get_blog_from_path(filepath: &str) -> Result<definitions::Blog>{
    let mut input = get_file_from_relative_path(&String::from(filepath))?;
    let date: DateTime<Utc> = get_modified_from_file(&input)?;
    let mut reader = BufReader::new(input);
    let title: String = get_title_from_reader(&mut reader)?;
    let mut content:String = String::new();
    let _ = reader.read_to_string(&mut content);
    let blog = definitions::Blog {
        title: title,
        content: content,
        date: date.timestamp_millis(),
    };
    Ok(blog)
}
pub fn get_file_object_from_path(filepath: &str) -> Result<definitions::BlogPreview> {
    let mut input = get_file_from_relative_path(&String::from(filepath))?;
    let modified_date: DateTime<Utc> = get_modified_from_file(&input)?;
    let (title,preview) = get_title_and_preview_from_file(&mut input)?;
    
    let blog = definitions::BlogPreview {
        title: title.to_string(),
        preview: preview.to_string(),
        date: modified_date.timestamp_millis(),
        id: Path::new(filepath).file_stem().unwrap().to_str().unwrap().into()
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
pub fn get_title_and_preview_from_file(input: &mut File) -> Result<(String,String)>{
    let mut reader = BufReader::new(input);

    let mut head_line = String::new();
    let _ = reader.read_line(&mut head_line)?;
    let title: String = head_line[2..head_line.len()].trim().into();
    let mut preview_line = String::new();
    let _ = reader.read_line(&mut preview_line)?;
    let preview:String = preview_line.trim().into();
    Ok((title,preview))
}
pub fn get_file_from_relative_path(mut filepath: &String) -> Result<File>{
    let mut path = scoped_join(definitions::FILEPATH, filepath)?;
    if Path::new(&path).exists() {
        return Ok(File::open(path)?)
    }
    let path_with_suffix: String = filepath.to_owned()+definitions::FILESUFFIX;
    path = scoped_join(definitions::FILEPATH, path_with_suffix)?;
    Ok(File::open(path)?)
    
}