use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::process::exit;

fn main() -> io::Result<()> {
    let entry = Path::new("./node_modules");

    if !entry.is_dir() {
        println!("No node_modules found.");
        exit(1)
    }

    let _ = visit_dirs(entry, &delete_module);

    Ok(())
}

/// Recursively walked the dirs deleting files.
/// Credit: https://doc.rust-lang.org/std/fs/fn.read_dir.html
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
                let _ = fs::remove_dir(path);
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn delete_module(entry: &DirEntry) {
    let _ = fs::remove_file(entry.path());
}
