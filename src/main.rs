use chrono::NaiveDate;
use handlebars::Handlebars;
use serde_derive::Serialize;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use toml::{Table, Value};
use walkdir::WalkDir;

#[derive(Clone, Serialize)]
struct Blog {
    config: Table,
    path: PathBuf,
}

#[derive(Clone, Serialize)]
struct BlogPage {
    title: String,
    blogs: Vec<Blog>,
    before: Option<usize>,
    after: Option<usize>,
}

fn main() {
    // make public folder
    let _ = fs::remove_dir_all("public");
    fs::create_dir("public").expect("could not make the folder \"public\"");

    //copy all static
    for e in WalkDir::new("static").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            let mut path = e.path();
            path = path.strip_prefix("static/").unwrap(); // should never fail
            let new_path = Path::new("public").join(path);
            let mut parent = new_path.to_path_buf();
            parent.pop();
            fs::create_dir_all(parent.clone())
                .unwrap_or_else(|_| panic!("Cant make the folders {}", parent.display()));
            fs::copy(e.path(), new_path).unwrap_or_else(|_| {
                panic!(
                    "failed to copy static/{} to public/{}",
                    path.display(),
                    path.display()
                )
            });
        }
    }

    // make template
    let mut reg = Handlebars::new();
    reg.register_template_string(
        "main",
        fs::read_to_string("template.html.hbs").expect("cant read template.html.hbs"),
    )
    .expect("cant make template");

    //format all pages
    for e in WalkDir::new("pages").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            let path = e.path();
            let contents = fs::read_to_string(path)
                .unwrap_or_else(|_| panic!("Cant read file {}", path.display()));
            let mut config = format!(
                "{}\ncontent = \"\"",
                contents
                    .split("+++")
                    .nth(1)
                    .unwrap_or_else(|| panic!("{} does not have a +++ section", path.display()))
            )
            .parse::<Table>()
            .unwrap();
            let content = contents
                .split("+++")
                .nth(2)
                .unwrap_or_else(|| panic!("{} does not have a +++ section", path.display()));
            let config_content = config.get_mut("content").unwrap();
            *config_content = Value::try_from(md_to_html(content)).unwrap();
            let path = Path::new("public")
                .join(path.strip_prefix("pages").unwrap())
                .with_extension("html");
            let mut parent = path.to_path_buf();
            parent.pop();
            fs::create_dir_all(parent.clone())
                .unwrap_or_else(|_| panic!("Cant make the folders {}", parent.display()));
            let mut file = File::create(path.clone())
                .unwrap_or_else(|_| panic!("Cant make file {}", path.display()));
            file.write_all(
                reg.render("main", &config)
                    .unwrap_or_else(|_| panic!("Cant render {}", path.display()))
                    .as_bytes(),
            )
            .unwrap_or_else(|_| panic!("Cant write to file {}", path.display()))
        }
    }

    //do the blogs
    if let Ok(blogs) = fs::read_dir("blogs") {
        fs::create_dir("public/blogs").expect("could not make the folder \"public/blogs\"");
        let mut all_blogs: Vec<Blog> = Vec::new();
        for i in blogs.flatten() {
            let path = i.path();
            let contents = fs::read_to_string(path.clone())
                .unwrap_or_else(|_| panic!("Cant read file {}", path.display()));
            let mut config = format!(
                "{}\ncontent = \"\"",
                contents
                    .split("+++")
                    .nth(1)
                    .unwrap_or_else(|| panic!("{} does not have a +++ section", path.display()))
            )
            .parse::<Table>()
            .unwrap();
            let content = contents
                .split("+++")
                .nth(2)
                .unwrap_or_else(|| panic!("{} does not have a +++ section", path.display()));
            let config_content = config.get_mut("content").unwrap();
            *config_content = Value::try_from(md_to_html(content)).unwrap();
            let path = path.strip_prefix("blogs").unwrap().with_extension("html");
            let mut file = File::create(Path::new("public/blogs").join(path.clone()))
                .unwrap_or_else(|_| panic!("Cant make file public/blogs/{}", path.display()));
            file.write_all(
                reg.render("main", &config)
                    .unwrap_or_else(|_| panic!("Cant render public/blogs/{}", path.display()))
                    .as_bytes(),
            )
            .unwrap_or_else(|_| panic!("Cant write to file public/blogs/{}", path.display()));
            all_blogs.push(Blog {
                config,
                path: path.to_path_buf(),
            });
        }
        all_blogs.sort_by(|a, b| {
            NaiveDate::parse_from_str(
                a.config["date"]
                    .as_str()
                    .unwrap_or_else(|| panic!("{} does not have a date", a.path.display())),
                "%Y-%m-%d",
            )
            .unwrap_or_else(|_| panic!("Cant convert {} date", a.path.display()))
            .cmp(
                &NaiveDate::parse_from_str(
                    b.config["date"]
                        .as_str()
                        .unwrap_or_else(|| panic!("{} does not have a date", b.path.display())),
                    "%Y-%m-%d",
                )
                .unwrap_or_else(|_| panic!("Cant convert {} date", b.path.display())),
            )
        });
        let mut i = 0;
        let blog_pages = all_blogs.chunks(10).map(|x| {
            let before = if i > 0 { Some(i - 1) } else { None };
            let after = if i < all_blogs.len() / 10 {
                Some(i + 1)
            } else {
                None
            };
            i += 1;
            BlogPage {
                title: "Blogs".to_string(),
                blogs: x.to_vec(),
                before,
                after,
            }
        });
        for (x, i) in blog_pages.enumerate() {
            let path_name = format!("public/blogs-{x}.html");
            let path = Path::new(&path_name);
            let mut file =
                File::create(path).unwrap_or_else(|_| panic!("Cant make file {}", path.display()));
            file.write_all(
                reg.render("main", &i)
                    .unwrap_or_else(|_| panic!("Cant render {}", path.display()))
                    .as_bytes(),
            )
            .unwrap_or_else(|_| panic!("Cant write to file {}", path.display()));
        }
    }
}

use emojicons::EmojiFormatter;
use pulldown_cmark::{html, Options, Parser};

pub fn md_to_html(input: &str) -> String {
    let input = EmojiFormatter(input).to_string();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
