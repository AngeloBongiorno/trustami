use std::path::Path;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

pub fn collect_valid_paths<P>(data_dir_path: P) -> Vec<PathBuf>
where P: AsRef<Path>  {

    let mut file_paths = Vec::new();
    let data_dir = fs::read_dir(data_dir_path).expect("An existing directory");

    for element in data_dir {
        let dir_entry = element.unwrap();
        let path = dir_entry.path();
        let file_type = dir_entry 
            .file_type()
            .unwrap();

        if file_type.is_file() {
            let filename = dir_entry.file_name().to_str().expect("Failed to convert filename").to_owned();
            let extension = get_extension_from_filename(&filename).expect("failed to get extension");
            if extension == "xml" {
                file_paths.push(path);
            }
        }
    }
    file_paths
}
