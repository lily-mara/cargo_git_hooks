use std::env;
use std::path::Path;
use std::os::unix;
use std::fs;
use std::io::ErrorKind;

fn main() {
    let manifest_dir = env::var("PWD")
        .expect("Failed to get working directory");
    let manifest_path = Path::new(&manifest_dir);

    let real_path = manifest_path.join(Path::new("hooks"));
    let git_path = manifest_path.join(".git").join("hooks");

    if cfg!(target_family = "unix") {
        match unix::fs::symlink(&real_path, &git_path) {
            Ok(_) => {}
            Err(e) => {
                if let ErrorKind::AlreadyExists = e.kind() {
                    fs::remove_file(&git_path).unwrap_or_else(|delete_err| {
                        panic!("Failed to delete existing git hooks folder: {:?}", delete_err);
                    });
                    unix::fs::symlink(&real_path, &git_path)
                        .unwrap_or_else(|inner_err| {
                            panic!("Failed to create symlink: {:?}", inner_err);
                        });
                } else {
                    panic!("Failed to create symlink: {:?}", e);
                }
            }
        }

    } else {
        panic!("Unsupported on this os");
    }
}
