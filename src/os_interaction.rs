use dirs;
use std::path::PathBuf;
use std::fs;
use std::error::Error;

fn create_index_collection_directory(mut data_directory: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    
    data_directory.push("trustami_index_collection");
    fs::create_dir(&data_directory)?;

    Ok(data_directory)
}

pub fn create_index_directory(dir_name: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let data_directory = dirs::data_local_dir().ok_or("Could not find OS user data directory.")?;
    let trustami_index_collection_path = data_directory.join("trustami_index_collection");

    if !trustami_index_collection_path.exists() {
        create_index_collection_directory(data_directory)?;
    }

    let index_path = trustami_index_collection_path.join(dir_name);
    fs::create_dir(&index_path)?;

    Ok(index_path)
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use crate::os_interaction::{create_index_collection_directory, create_index_directory};

    #[test]
    fn index_collection_directory_is_created() {

        let temp = tempdir().unwrap();
        let base_path = temp.path().to_path_buf();

        let path = create_index_collection_directory(base_path).unwrap();

        assert!(path.exists());
        assert!(path.is_dir());

    }

    #[test]
    fn index_collection_directory_is_not_created_when_already_exists() {

        let temp = tempdir().unwrap();
        let base_path = temp.path().to_path_buf();
        let dir = base_path.join("trustami_index_collection");

        std::fs::create_dir_all(&dir).unwrap();

        let result = create_index_collection_directory(base_path);

        assert!(result.is_err());
    }


    #[test]
    fn index_directory_is_created() {

        let temp = tempdir().unwrap();
        let base_path = temp.path().to_path_buf();
        let dir = base_path.join("trustami_index_collection");

        std::fs::create_dir_all(&dir).unwrap();

        let index_path = create_index_directory(dir).unwrap();

        assert!(index_path.exists());
        assert!(index_path.is_dir());
    }
}
