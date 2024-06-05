use std::io::{Result, BufReader};

pub fn content_to_html(content: &str) -> Result<String>{
    let lines = content.lines();
    let mut output = String::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with("#") {
            let (heading,title) = line.split_at(line.find(' ').unwrap());
            output.push_str(&get_html_heading(heading, title));
            continue;
        }
        output.push_str(&get_paragraph(line));
    }
    Ok(output)
}
fn get_html_heading(heading: &str,title: &str) -> String{
    let size = heading.len();
    return format!("<h{size}> {content} </h{size}>",size=size,content=title)
}
fn get_paragraph(content: &str) -> String{
    return format!("<p> {content} </p>");
}

