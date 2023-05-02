use std::{path::Path, collections::HashMap, fs::{FileType, read_to_string}};

/// Builds file tree
/// path_string must point to directory containing .chr
pub fn build_tree(path_string: String) -> anyhow::Result<HashMap> {
    let path = Path::new(&path_string);

    let mut files: HashMap<String, String> = HashMap::new();
    let mut iter = path.read_dir()?;

    while let Some(Ok(entry)) = iter.next() {
        let entry_type = entry.file_type()?;
        let entry_path = entry.path().to_str()?.to_string();

        if entry_type.is_file() {
            files.insert(entry_path, read_to_string(entry_path));
        } else if entry_type.is_dir() {

        }
    };

    Ok(Tree { files })
}

pub struct Tree {
    pub files: HashMap<String, String>, // File path (relative to .chr) to file content mapping
}
