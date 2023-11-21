use std::fs;
use std::path::Path;

fn main() {
    let path = String::from(format!("C:/Users/{}/AppData/Local/Temp", whoami::username())); // user temp
    clean(path);
    let path = String::from("C:/Windows/Temp"); // windows temp
    clean(path);
    let path = String::from("C:/Windows/LiveKernelReports"); // kernel reports
    clean(path);
    let path = String::from("C:/Windows/Downloaded Program Files"); // trash for internet explorer, may also contain viruses
    clean(path);
    let path = String::from("C:/Windows/SoftwareDistribution"); // old windows updates
    clean(path);
    let path = String::from("C:/Windows/Minidump"); // dumps
    clean(path);
    let path = String::from("C:/Windows/Prefetch"); // only trash
    clean(path);
    let path = String::from("C:/Windows/Logs"); // logs
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