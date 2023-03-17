use walkdir::WalkDir;
use std::fs;
use std::path::Path;

fn main() {
    let _ = fs::create_dir("public");
    for e in WalkDir::new("static").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            let mut path = e.path();
            path = path.strip_prefix("static/").unwrap(); // should never fail
            fs::copy(e.path(), Path::new("public").join(path)).expect(&format!("failed to copy static/{} to public/{}", path.display(), path.display()));
        }
    }
}
