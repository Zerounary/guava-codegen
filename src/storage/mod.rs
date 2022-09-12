use std::env;
use std::{ path::Path, fs,};
use lazy_static::lazy_static;

lazy_static! {
        // connect database
    pub static ref PROJECT_NAME: String =
        env::var("PROJECT_NAME").expect("No PROJECT_NAME provided");
}

pub fn gen_root_file(file: &str, content: String ) {
    let path = format!("../{}/", PROJECT_NAME.clone());
    let dest_path = Path::new(&path).join(file);
    let dir_path = dest_path.parent().unwrap();
    if !dir_path.exists() {
        fs::create_dir_all(dir_path);
    }
    fs::write(
        &dest_path,
        content
        ).unwrap();
}

pub fn gen_file(file: &str, content: String ) {
    let path = format!("../{}/src", PROJECT_NAME.clone());
    let dest_path = Path::new(&path).join(file);
    let dir_path = dest_path.parent().unwrap();
    if !dir_path.exists() {
        fs::create_dir_all(dir_path);
    }
    fs::write(
        &dest_path,
        content
        ).unwrap();
}
