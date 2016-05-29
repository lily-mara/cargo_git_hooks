use std::env;
use std::path::Path;
use std::os::unix;

fn main() {
    let manifest_dir = env::var("PWD")
        .expect("Failed to get working directory");
    let manifest_path = Path::new(&manifest_dir);

    let real_path = manifest_path.join(Path::new("hooks"));
    let git_path = manifest_path.join(".git").join("hooks");

    if cfg!(target_family = "unix") {
        unix::fs::symlink(real_path, git_path).unwrap_or_else(|why| {
            println!("Failed to create symlink {:?}", why.kind());
        });
    } else {
        panic!("Unsupported on this os");
    }
}
