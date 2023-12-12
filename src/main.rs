use std::fs;
use std::path::Path;

fn main() {
    let pathes = vec![
        String::from(format!("C:/Users/{}/AppData/Local/Temp", whoami::username())), // user temp
        String::from("C:/Windows/Temp"), // windows temp
        String::from("C:/Windows/LiveKernelReports"), // kernel reports
        String::from("C:/Windows/Downloaded Program Files"), // trash for internet explorer, may also contain viruses
        String::from("C:/Windows/SoftwareDistribution"), // old windows updates
        String::from("C:/Windows/Prefetch"), // only trash
        String::from("C:/Windows/Logs"), // logs
        String::from(format!("C:/Users/{}/AppData/Local/DBG", whoami::username())), // PDB cache
        String::from(format!("C:/Users/{}/AppData/Local/Microsoft/Windows/Explorer", whoami::username())), // thumbnail cache
        String::from("C:/ProgramData/Microsoft/Windows/WER"), // Windows reports
        // dumps
        String::from("C:/Windows/Minidump"),
        String::from(format!("C:/users/{}/appdata/local/CrashDumps", whoami::username())),
        // temporary driver unpacking folders
        String::from("C:/AMD"),
        String::from("C:/Intel"),
    ];

    for path in pathes.iter() {
        clean(path);
    }
}

fn clean(path: &String) {
    let dir = Path::new(&path);

    if dir.exists() {
        let files = dir.read_dir().unwrap();
        for file in files {
            let file_path = file.unwrap().path();
            if let Err(_) = fs::remove_file(file_path.clone()) {}
            if let Err(_) = fs::remove_dir_all(file_path) {}
        }
    }
}