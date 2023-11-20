use std::fs;
use std::path::Path;

fn main() {
    let path = String::from(format!("C:/Users/{}/AppData/Local/Temp", whoami::username()));
    clean(path);
    let path = String::from("C:/Windows/Temp");
    clean(path);
}

fn clean(path: String) {
    let dir = Path::new(&path);

    let files = dir.read_dir().unwrap();

    for file in files {
        let file_path = file.unwrap().path();
        match fs::remove_file(file_path.clone()) {
            Ok(_) => (),
            Err(_) => (),
        }
        match fs::remove_dir_all(file_path) {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}