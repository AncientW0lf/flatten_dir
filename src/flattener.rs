use std::fs::{self, DirEntry, Metadata};
use std::{io, path};

pub fn flatten(dir: &str) -> Result<(), io::Error> {
    let dir_path = path::Path::new(dir);
    let files = get_files(dir)?;

    for file in files {
        let path = path::Path::new(&file);
        let filename_only = path
            .file_name()
            .unwrap()
            .to_str()
            .expect("Invalid filename found. Filenames can only have unicode characters.");

        fs::rename(path, dir_path.join(filename_only))?;

        let parent_dir = path.parent().unwrap();

        if parent_dir.read_dir()?.count() == 0 {
            fs::remove_dir(parent_dir)?;
        }
    }

    return Ok(());
}

fn get_files(dir: &str) -> Result<Vec<String>, io::Error> {
    let mut files: Vec<String> = Vec::new();
    let dir_info = fs::read_dir(dir)?;
    for file_info in dir_info {
        let file_info: DirEntry = file_info?;
        let file_meta: Metadata = file_info.metadata()?;

        let file_path = String::from(file_info.path().to_str().expect("Could not get file path!"));

        if file_meta.is_dir() {
            files.extend(get_files(&file_path)?);
        } else {
            files.push(file_path);
        }
    }

    return Ok(files);
}
