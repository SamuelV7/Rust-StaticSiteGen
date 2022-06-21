use std::path::{Path};
use std::fs::{self, DirEntry};
use pulldown_cmark::{Parser, html};

// read mark down files in directory under assets probably
struct HtmlPage<'a> {
    head: &'a str,
    body: &'a str,
    footer: &'a str,
}
struct MarkDownFiles{
    name : String,
    contents : String,
    path: String
}
impl std::fmt::Display for MarkDownFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n {} \n {} \n {}", self.name, self.path, self.contents)
    }
}
impl HtmlPage<'_> {
    fn new() -> Self {
        return HtmlPage {
            head: r#"<!DOCTYPE html>
<html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Document</title>
        </head>"#,
            body: r#"<body></body>"#,
            footer: r#"</html>"#,
        };
    }
}
impl std::fmt::Display for HtmlPage<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n {} \n {} \n {}", self.head, self.body, self.footer)
    }
}

fn files_to_string(path: &Path) -> String{
    fs::read_to_string(path).expect("Something wrong with reading file")
}

fn files_in_dir(path : &Path) -> Vec<Result<DirEntry, std::io::Error>>{
    let dir  = fs::read_dir(path).expect("Something wrong with reading from directory");
    let md_files: Vec<Result<DirEntry, std::io::Error>> = dir
    .into_iter()
    .filter(|file| {
        match file.as_ref().unwrap().path().extension() {
            None => false,
            Some(ext) => ext == "md"
        }
    }).collect();
    md_files
}
fn markdown_to_html(md_file : MarkDownFiles)-> String{
    let parser = Parser::new(md_file.contents.as_str());
    let mut html_string =  String::new();
    html::push_html(&mut html_string, parser);
    return html_string
}
fn main() {
    let files = files_in_dir(Path::new("src/markdown"));

    let string_of_files: Vec<Result<MarkDownFiles, std::io::Error>> = files.into_iter()
    .map(|file|{
        match file {
            Ok(dir) => {
                let mark = MarkDownFiles{
                    name: String::from(dir.file_name().to_str().unwrap()),
                    path: String::from(dir.path().to_str().unwrap()),
                    contents: String::from(files_to_string(dir.path().as_path())),
                };
                Result::Ok(mark)
            },
            Err(err) => {Result::Err(err)},
        }
    }).collect();
    
    for file in string_of_files{
        match file {
            Ok(md) => {
                let html_output = markdown_to_html(md);
                println!("{}", html_output);
                html_output
            },
            Err(_) => String::from("ERR"),
        };
    }
}
