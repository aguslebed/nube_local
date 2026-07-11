use std::path::PathBuf;
use nube_local::fs_ops;

const BASE_DIR: &str = "";
fn get_root_path() -> PathBuf {
    dirs::home_dir()
        .expect("no se pudo encontrar el home")
        .join("nube-data")
}

fn main() {
    let root_path = get_root_path();
    fs_ops::create_new_dir(&root_path, "test_dir").unwrap();
    println!("Root path: {:?}", root_path);
}