use std::{collections::HashMap, fs::read_to_string, path::Path};

/// Builds file tree
/// path_string must point to directory containing .chr
pub fn build_tree(path_string: String) -> anyhow::Result<HashMap<String, String>> {
    let path = Path::new(&path_string);

    let mut files: HashMap<String, String> = HashMap::new();
    let mut iter = path.read_dir()?;

    while let Some(Ok(entry)) = iter.next() {
        let entry_type = entry.file_type()?;
        let entry_path = match entry.path().to_str() {
            Some(val) => val.to_string(),
            None => {
                anyhow::bail!("Unable to fetch entry path");
            }
        };

        if entry_type.is_file() {
            files.insert(entry_path.clone(), read_to_string(entry_path)?);
        } else if entry_type.is_dir() {
            files.extend(build_tree(entry_path)?);
        };
    }

    Ok(files)
}
