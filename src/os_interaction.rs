//use dirs;
use anyhow::{self, Context};
use std::fs::{self, File};
use std::io::{BufRead, ErrorKind};
use std::path::{Path, PathBuf};

const APPLICATION_DATA_DIRECTORY_NAME: &str = "trustami_application_data";
const INDEX_FILENAME: &str = "index.json";

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

#[cfg(test)]
mod tests {
    use crate::os_interaction::{
        APPLICATION_DATA_DIRECTORY_NAME, INDEX_FILENAME, create_application_data_directory,
        create_index_directory, create_index_file,
    };
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn application_data_directory_is_created() {
        let temp = tempdir().unwrap();
        let base_path = temp.path().to_path_buf();

        let path = create_application_data_directory(base_path).unwrap();

        assert!(path.exists());
        assert!(path.is_dir());
    }

    #[test]
    fn application_data_directory_errors_if_already_exists() {
        let temp = tempdir().unwrap();
        let base_path = temp.path().to_path_buf();

        fs::create_dir(base_path.join(APPLICATION_DATA_DIRECTORY_NAME)).unwrap();

        let result = create_application_data_directory(base_path);

        assert!(result.is_err());
    }

    #[test]
    fn index_directory_is_created() {
        let temp = tempdir().unwrap();
        let base_path = temp.path().to_path_buf();
        let data_dir = base_path.join(APPLICATION_DATA_DIRECTORY_NAME);
        std::fs::create_dir_all(&data_dir).unwrap();
        let index_dir_name = "new_index_dir";

        let index_path = create_index_directory(data_dir, index_dir_name).unwrap();

        assert!(index_path.exists());
        assert!(index_path.is_dir());
    }

    #[test]
    fn index_directory_creation_errors_if_directory_exists() {
        let temp = tempdir().unwrap();
        let base_path = temp.path().to_path_buf();
        let data_dir = base_path.join(APPLICATION_DATA_DIRECTORY_NAME);
        let index_dir_name = "my_index_dir";
        let index_path = data_dir.join(index_dir_name);
        std::fs::create_dir_all(&index_path).unwrap();

        let result = create_index_directory(data_dir, index_dir_name);

        assert!(result.is_err());
    }

    #[test]
    fn index_file_is_created() {
        let temp = tempdir().unwrap();
        let base_path = temp.path().to_path_buf();
        let index_dir_name = "my_index_dir";
        let expected_file_path = base_path
            .join(APPLICATION_DATA_DIRECTORY_NAME)
            .join(index_dir_name)
            .join(INDEX_FILENAME);

        // useless for this test case
        let user_input = b"y";

        let result = create_index_file(base_path, index_dir_name, &user_input[..]);

        assert!(result.is_ok());
        assert!(expected_file_path.exists());
        assert!(expected_file_path.is_file());
    }

    #[test]
    fn index_file_is_overwritten() {
        let temp = tempdir().unwrap();
        let base_path = temp.path().to_path_buf();
        let index_dir_name = "my_index_dir";
        let expected_file_path = base_path
            .join(APPLICATION_DATA_DIRECTORY_NAME)
            .join(index_dir_name)
            .join(INDEX_FILENAME);
        //let mut creation1 = SystemTime::now();
        //let mut creation2 = SystemTime::now();
        let user_input = b"y";

        let result1 = create_index_file(base_path.clone(), index_dir_name, &user_input[..]);
        //let _ = result1.as_ref().inspect(|f| {
        //    creation1 = f.metadata().unwrap().created().unwrap();
        //});

        let result2 = create_index_file(base_path.clone(), index_dir_name, &user_input[..]);
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
        let temp = tempdir().unwrap();
        let base_path = temp.path().to_path_buf();
        let index_dir_name = "my_index_dir";
        let expected_file_path = base_path
            .join(APPLICATION_DATA_DIRECTORY_NAME)
            .join(index_dir_name)
            .join(INDEX_FILENAME);
        let user_input = b"n";

        let result1 = create_index_file(base_path.clone(), index_dir_name, &user_input[..]);
        let result2 = create_index_file(base_path.clone(), index_dir_name, &user_input[..]);

        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert!(expected_file_path.exists());
        assert!(expected_file_path.is_file());
    }
}
