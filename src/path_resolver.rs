use anyhow;
use anyhow::Context;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

pub fn collect_valid_paths<P>(data_dir_path: P) -> Result<Vec<PathBuf>, anyhow::Error>
where
    P: AsRef<Path>,
{
    let mut file_paths = Vec::new();
    let data_dir = fs::read_dir(data_dir_path).context("Failed to find specified directory.")?;

    for element in data_dir {
        let dir_entry = element.unwrap();
        let path = dir_entry.path();
        let file_type = dir_entry
            .file_type()
            .with_context(|| format!("Could not retrieve file type for {}", { path.display() }))?;

        if file_type.is_file() {
            let filename = dir_entry
                .file_name()
                .to_str()
                .context("Failed to convert filename to string")?
                //.expect("Failed to convert filename")
                .to_owned();
            if let Some(extension) = get_extension_from_filename(&filename) {
                if extension == "xml" {
                    println!("Obtained file extension for: {}", filename);
                    file_paths.push(path);
                } else {
                    eprintln!("File extension for: {} is not supported.", filename);
                }
            } else {
                eprintln!("Could not obtain file extension for: {}.", filename);
            }
        }
    }
    Ok(file_paths)
}
