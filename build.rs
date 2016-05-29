use std::env;
use std::path::Path;
use std::os::unix;
use std::fs;

fn main() {
    let manifest_dir = env::var("PWD")
        .expect("Failed to get working directory");
    let manifest_path = Path::new(&manifest_dir);

    let real_path = manifest_path.join(Path::new("hooks"));
    let git_path = manifest_path.join(".git").join("hooks");

    if git_path.is_file() {
        fs::remove_file(&git_path).unwrap_or_else(|e| {
            panic!("Failed to delete existing git hooks file: {:?}", e);
        });
    } else if git_path.is_dir() {
        fs::remove_dir_all(&git_path).unwrap_or_else(|e| {
            panic!("Failed to delete existing git hooks folder: {:?}", e);
        });
    }

    if cfg!(target_family = "unix") {
        unix::fs::symlink(&real_path, &git_path).unwrap_or_else(|why| {
            panic!("Failed to create symlink {:?}", why.kind());
        });
    } else {
        panic!("Unsupported on this os");
    }
}
