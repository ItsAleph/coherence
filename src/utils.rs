use std::{collections::HashMap, fs::read_to_string, path::Path, io::ErrorKind};

/// Builds file tree
/// path_string must point to directory containing .chr
pub fn build_tree(path_string: String, ignore: &Vec<String>) -> anyhow::Result<HashMap<String, String>> {
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
            let entry_content = match read_to_string(&entry_path) {
                Ok(content) => content,
                Err(e) => {
                    if e.kind() == ErrorKind::InvalidData {
                        println!("ERROR: Unable to read contents of '{}': binary files are not supported yet.", entry_path);
                        continue;
                    };
                    anyhow::bail!("Unable to read file contents");
                },
            };
    
            files.insert(entry_path.clone(), entry_content);
        } else if entry_type.is_dir() {
            files.extend(build_tree(entry_path, &ignore)?);
        };
    };

    Ok(files)
}

pub fn parse_ignore(root: String) -> anyhow::Result<Vec<String>> {
    let mut ignores = vec![];
    let content = read_to_string(root)?;
    let mut lines = content.lines();

    for line in lines {
        if line.is_empty() { continue; };
        if line.starts_with('#') { continue; };

        ignores.push(line.to_string());
    };

    Ok(ignores)
}