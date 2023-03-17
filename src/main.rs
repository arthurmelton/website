use walkdir::WalkDir;
use std::fs::{self, File};
use std::path::Path;
use handlebars::Handlebars;
use std::io::Write;
use toml::{Table, Value};

fn main() {
    // make public folder
    let _ = fs::remove_dir_all("public");
    fs::create_dir("public").expect("could not make the folder \"public\"");

    //copy all static
    for e in WalkDir::new("static").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            let mut path = e.path();
            path = path.strip_prefix("static/").unwrap(); // should never fail
            fs::copy(e.path(), Path::new("public").join(path)).expect(&format!("failed to copy static/{} to public/{}", path.display(), path.display()));
        }
    }

    // make template
    let mut reg = Handlebars::new();
    reg.register_template_string("main", fs::read_to_string("template.html.hbs").expect("cant read template.html.hbs")).expect("cant make template");

    //format all pages
    for e in WalkDir::new("pages").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            let path = e.path();
            let contents = fs::read_to_string(path).expect(&format!("Cant read file {}", path.display()));
            let mut config = format!("{}\ncontent = \"\"", contents.split("+++").nth(1).expect(&format!("{} does not have a +++ section", path.display()))).parse::<Table>().unwrap();
            let content = contents.split("+++").nth(2).expect(&format!("{} does not have a +++ section", path.display()));
            let config_content = config.get_mut("content").unwrap();
            *config_content = Value::try_from(md_to_html(content)).unwrap();
            let path = Path::new("public").join(path.strip_prefix("pages").unwrap()).with_extension("html");
            let mut file = File::create(path.clone()).expect(&format!("Cant make file {}", path.display()));
            file.write_all(reg.render("main", &config).expect(&format!("Cant render {}", path.display())).as_bytes()).expect(&format!("Cant write to file {}", path.display()))
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
