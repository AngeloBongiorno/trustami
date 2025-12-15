//use dirs;
use anyhow::{self, Context};
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{BufRead, ErrorKind};
use std::path::{Path, PathBuf};
use thiserror::Error;

const APPLICATION_DATA_DIRECTORY_NAME: &str = "trustami_application_data";
const INDEX_FILENAME: &str = "index.json";

#[derive(Debug, Error)]
enum FolderStructureError<'a> {
    #[error("The folder {} was missing", .0)]
    MissingFolderError(&'a str),
}

fn create_application_data_directory(
    mut user_data_directory: PathBuf,
) -> Result<PathBuf, anyhow::Error> {
    user_data_directory.push(APPLICATION_DATA_DIRECTORY_NAME);
    fs::create_dir(&user_data_directory).with_context(|| {
        format!(
            "Could not create {} directory.",
            APPLICATION_DATA_DIRECTORY_NAME
        )
    })?;

    Ok(user_data_directory)
}

fn create_index_directory<P>(
    application_data_path: PathBuf,
    dir_name: P,
) -> Result<PathBuf, anyhow::Error>
where
    P: AsRef<Path>,
{
    let index_path = application_data_path.join(dir_name);

    fs::create_dir(&index_path)
        .with_context(|| format!("Could not create directory at path: {:?}", index_path))?;

    Ok(index_path)
}

pub fn create_index_file<P, R>(
    user_data_directory: PathBuf,
    index_name: P,
    mut reader: R,
) -> Result<File, anyhow::Error>
where
    P: AsRef<Path>,
    R: BufRead,
{
    let application_data_path = user_data_directory.join(APPLICATION_DATA_DIRECTORY_NAME);
    if !application_data_path.exists() {
        let _ = create_application_data_directory(user_data_directory)?;
    }
    let index_directory = application_data_path.join(&index_name);
    if !index_directory.exists() {
        let _ = create_index_directory(application_data_path, index_name)?;
    }

    let file_path = index_directory.join(INDEX_FILENAME);
    //let file_handle = File::create_new(&file_path).context("Failed to create an index file.")?;

    let file_handle = match File::create_new(&file_path) {
        Ok(file_handle) => file_handle,
        Err(err) => match err.kind() {
            ErrorKind::AlreadyExists => {
                println!(
                    "An existing index with the same name was found: do you want to replace it? [y/N]"
                );
                let mut buffer = String::new();
                loop {
                    buffer.clear();
                    reader.read_line(&mut buffer)?;
                    let input = buffer.trim().to_lowercase();
                    if input == "y" {
                        break File::create(&file_path)?;
                    } else if input == "n" {
                        return Err(err).context("Operation aborted by user.");
                    }
                    println!("Please insert [y/N] to replace existing index or abort operation.");
                }
            }
            _ => return Err(err).context("Generic error while creating the index file"),
        },
    };

    Ok(file_handle)
}

pub fn get_index_names(user_data_directory: PathBuf) -> Result<Vec<OsString>, anyhow::Error> {
    let application_data_path = user_data_directory.join(APPLICATION_DATA_DIRECTORY_NAME);
    if application_data_path.exists() && application_data_path.is_dir() {
        let dir_entries = fs::read_dir(application_data_path)
            .context("Failed to read application data directory.")?;

        let index_names: Vec<OsString> = dir_entries
            .filter_map(|e| match e {
                Ok(entry) => match entry.file_type() {
                    Ok(file_type) => {
                        if file_type.is_dir() {
                            Some(entry.file_name())
                        } else {
                            None
                        }
                    }
                    Err(ref err) => {
                        eprintln!(
                            "Could not get file type of entry: {:?}. Error: {}",
                            entry, err
                        );
                        None
                    }
                },
                Err(ref err) => {
                    eprintln!(
                        "An error occured while reading entry: {:?}. Error: {}",
                        e, err
                    );
                    None
                }
            })
            .collect();
        Ok(index_names)
    } else {
        Err(FolderStructureError::MissingFolderError(
            APPLICATION_DATA_DIRECTORY_NAME,
        ))
        .context("Could not find application data folder while retrieving index names.")
    }
}

#[cfg(test)]
mod tests {
    use crate::os_interaction::{
        APPLICATION_DATA_DIRECTORY_NAME, INDEX_FILENAME, create_application_data_directory,
        create_index_directory, create_index_file, get_index_names,
    };
    use std::{
        fs::{self, File},
        path::PathBuf,
    };
    use tempfile::{TempDir, tempdir};

    fn create_fake_user_data_path() -> (TempDir, PathBuf) {
        let temp = tempdir().unwrap();
        let path = temp.path().to_path_buf();

        (temp, path)
    }

    #[test]
    fn application_data_directory_is_created() {
        let (_temp, fake_user_data_path) = create_fake_user_data_path();

        let path = create_application_data_directory(fake_user_data_path).unwrap();

        assert!(path.exists());
        assert!(path.is_dir());
    }

    #[test]
    fn application_data_directory_errors_if_already_exists() {
        let (_temp, fake_user_data_path) = create_fake_user_data_path();

        fs::create_dir(fake_user_data_path.join(APPLICATION_DATA_DIRECTORY_NAME)).unwrap();

        let result = create_application_data_directory(fake_user_data_path);

        assert!(result.is_err());
    }

    #[test]
    fn index_directory_is_created() {
        let (_temp, fake_user_data_path) = create_fake_user_data_path();

        let data_dir = fake_user_data_path.join(APPLICATION_DATA_DIRECTORY_NAME);
        std::fs::create_dir_all(&data_dir).unwrap();
        let index_dir_name = "new_index_dir";

        let index_path = create_index_directory(data_dir, index_dir_name).unwrap();

        assert!(index_path.exists());
        assert!(index_path.is_dir());
    }

    #[test]
    fn index_directory_creation_errors_if_directory_exists() {
        let (_temp, fake_user_data_path) = create_fake_user_data_path();

        let data_dir = fake_user_data_path.join(APPLICATION_DATA_DIRECTORY_NAME);
        let index_dir_name = "my_index_dir";
        let index_path = data_dir.join(index_dir_name);
        std::fs::create_dir_all(&index_path).unwrap();

        let result = create_index_directory(data_dir, index_dir_name);

        assert!(result.is_err());
    }

    #[test]
    fn index_file_is_created() {
        let (_temp, fake_user_data_path) = create_fake_user_data_path();

        let index_dir_name = "my_index_dir";
        let expected_file_path = fake_user_data_path
            .join(APPLICATION_DATA_DIRECTORY_NAME)
            .join(index_dir_name)
            .join(INDEX_FILENAME);

        // useless for this test case
        let user_input = b"y";

        let result = create_index_file(fake_user_data_path, index_dir_name, &user_input[..]);

        assert!(result.is_ok());
        assert!(expected_file_path.exists());
        assert!(expected_file_path.is_file());
    }

    #[test]
    fn index_file_is_overwritten() {
        let (_temp, fake_user_data_path) = create_fake_user_data_path();

        let index_dir_name = "my_index_dir";
        let expected_file_path = fake_user_data_path
            .join(APPLICATION_DATA_DIRECTORY_NAME)
            .join(index_dir_name)
            .join(INDEX_FILENAME);
        //let mut creation1 = SystemTime::now();
        //let mut creation2 = SystemTime::now();
        let user_input = b"y";

        let result1 =
            create_index_file(fake_user_data_path.clone(), index_dir_name, &user_input[..]);
        //let _ = result1.as_ref().inspect(|f| {
        //    creation1 = f.metadata().unwrap().created().unwrap();
        //});

        let result2 =
            create_index_file(fake_user_data_path.clone(), index_dir_name, &user_input[..]);
        //let _ = result2.as_ref().inspect(|f| {
        //    creation2 = f.metadata().unwrap().created().unwrap();
        //});

        //dbg!(creation1, creation2);

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        //assert!(creation1 < creation2);
        assert!(expected_file_path.exists());
        assert!(expected_file_path.is_file());
    }

    #[test]
    fn index_file_is_not_overwritten() {
        let (_temp, fake_user_data_path) = create_fake_user_data_path();

        let index_dir_name = "my_index_dir";
        let expected_file_path = fake_user_data_path
            .join(APPLICATION_DATA_DIRECTORY_NAME)
            .join(index_dir_name)
            .join(INDEX_FILENAME);
        let user_input = b"n";

        let result1 =
            create_index_file(fake_user_data_path.clone(), index_dir_name, &user_input[..]);
        let result2 =
            create_index_file(fake_user_data_path.clone(), index_dir_name, &user_input[..]);

        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert!(expected_file_path.exists());
        assert!(expected_file_path.is_file());
    }

    #[test]
    fn retrieve_indexes() {
        let (_temp, fake_user_data_path) = create_fake_user_data_path();

        let data_dir_path = fake_user_data_path.join(APPLICATION_DATA_DIRECTORY_NAME);
        fs::create_dir(&data_dir_path).unwrap();

        for i in 0..5 {
            fs::create_dir(data_dir_path.join(format!("index_{i}"))).unwrap();
        }

        let names = get_index_names(fake_user_data_path);

        assert!(names.is_ok());
        dbg!(&names);
        assert_eq!(names.unwrap().len(), 5);
    }

    #[test]
    fn retrieve_indexes_ignores_files() {
        let (_temp, fake_user_data_path) = create_fake_user_data_path();

        let data_dir_path = fake_user_data_path.join(APPLICATION_DATA_DIRECTORY_NAME);
        fs::create_dir(&data_dir_path).unwrap();

        for i in 0..3 {
            fs::create_dir(data_dir_path.join(format!("index_{i}"))).unwrap();
        }
        for i in 0..2 {
            File::create_new(data_dir_path.join(format!("test_{i}.txt"))).unwrap();
        }

        let names = get_index_names(fake_user_data_path);

        assert!(names.is_ok());
        dbg!(&names);
        assert_eq!(names.unwrap().len(), 3);
    }
}
