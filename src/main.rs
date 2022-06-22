use pulldown_cmark::{html, Parser};
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

// read mark down files in directory under assets probably
struct HtmlPage {
    head: String,
    body_open: String,
    body_closer: String,
    footer: String,
}
struct MarkDownFiles {
    name: String,
    contents: String,
    path: PathBuf,
}
struct HtmlOfMD {
    name: String,
    contents: HtmlPage,
}
impl std::fmt::Display for MarkDownFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\n {} \n {} \n {}",
            self.name,
            self.path.to_str().unwrap(),
            self.contents
        )
    }
}
impl HtmlPage {
    fn new() -> Self {
        return HtmlPage {
            head: r#"<!DOCTYPE html>
<html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Document</title>
        </head>"#
                .to_string(),
            body_open: r#"<body>"#.to_string(),
            body_closer: r#"</body>"#.to_string(),
            footer: r#"</html>"#.to_string(),
        };
    }
    fn add_to_body(mut self, html_items: String) -> Self {
        self.body_open = self.body_open.to_owned() + &html_items;
        self
    }
    fn link_html_tag(location: String) -> String {
        r#"<li><a href=""#.to_owned()
            + location.as_str()
            + r#"">"#
            + location.as_str()
            + r#"</a></li>"#
    }
}
impl std::fmt::Display for HtmlPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n {} \n {} \n {} \n {}",
            self.head, self.body_open, self.body_closer, self.footer
        )
    }
}

fn files_to_string(path: &Path) -> String {
    fs::read_to_string(path).expect("Something wrong with reading file")
}

fn files_in_dir(path: &Path, file_type: &str) -> Vec<Result<DirEntry, std::io::Error>> {
    let dir = fs::read_dir(path).expect("Something wrong with reading from directory");
    let md_files: Vec<Result<DirEntry, std::io::Error>> = dir
        .into_iter()
        .filter(|file| match file.as_ref().unwrap().path().extension() {
            None => false,
            Some(ext) => ext == file_type,
        })
        .collect();
    md_files
}

fn markdown_to_html(md_file: MarkDownFiles) -> HtmlOfMD {
    let parser = Parser::new(md_file.contents.as_str());
    let mut html_string = String::new();
    html::push_html(&mut html_string, parser);
    HtmlOfMD {
        name: md_file
            .path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        contents: HtmlPage::new().add_to_body(html_string),
    }
}

fn save_html(html: HtmlOfMD, location: &Path) -> Result<(), std::io::Error> {
    let name = Path::file_stem(Path::new(&html.name)).unwrap().to_str();
    let mut out = location.join(Path::new(name.unwrap()));
    out.set_extension("html");
    fs::write(out, html.contents.to_string())?;
    Ok(())
}

fn create_contents_page(files_to_link: Vec<Result<DirEntry, std::io::Error>>) {
    let string_of_paths: Vec<String> = files_to_link
        .into_iter()
        .map(|file| {
            let the_path = String::from(file.unwrap().file_name().to_str().unwrap());
            let linky = HtmlPage::link_html_tag(the_path);
            linky
        })
        .collect();

    let out = string_of_paths.into_iter().reduce(|accum, item| {
        let plus = accum + "\n" + &item + "\n";
        plus
    });
    let html_page = HtmlPage::new();
    let out = html_page.add_to_body(out.unwrap()).to_string();
    // println!("TEst {}", out.unwrap());
    fs::write("src/assets/html/index.html", out).unwrap();
}
fn main() {
    let files = files_in_dir(Path::new("src/markdown"), "md");

    let string_of_files: Vec<Result<MarkDownFiles, std::io::Error>> = files
        .into_iter()
        .map(|file| {
            file.and_then(|file1| {
                let mark = MarkDownFiles {
                    name: String::from(file1.file_name().to_str().unwrap()),
                    path: file1.path(),
                    contents: String::from(files_to_string(file1.path().as_path())),
                };
                Result::Ok(mark)
            })
        })
        .collect();

    for file in string_of_files {
        let result = file.and_then(|the_file| {
            let html_output = markdown_to_html(the_file);
            Result::Ok(html_output)
        });
        let _ = result.and_then(|html| {
            let out = save_html(html, Path::new("src/assets/html"));
            Result::Ok(out)
        });
    }
    let html_files = files_in_dir(Path::new("src/assets/html"), "html");
    create_contents_page(html_files);
}