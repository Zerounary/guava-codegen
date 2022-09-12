
use std::{ path::Path, fs,};

pub fn gen_root_file(file: &str, content: String ) {
    let dest_path = Path::new("../gen/").join(file);
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
    let dest_path = Path::new("../gen/src/").join(file);
    let dir_path = dest_path.parent().unwrap();
    if !dir_path.exists() {
        fs::create_dir_all(dir_path);
    }
    fs::write(
        &dest_path,
        content
        ).unwrap();
}
