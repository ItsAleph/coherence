use std::{collections::HashMap, fs::read_to_string, io::ErrorKind, path::Path};

/// Builds file tree
/// path_string must point to directory containing .chr
pub fn build_tree(
    path_string: String,
    ignore: &Vec<String>,
) -> anyhow::Result<HashMap<String, String>> {
    let path = Path::new(&path_string);

    let mut files: HashMap<String, String> = HashMap::new();
    let mut iter = path.read_dir()?;

    while let Some(Ok(entry)) = iter.next() {
        let entry_type = entry.file_type()?;
        let entry_path = match entry.path().to_str() {
            Some(val) => val.to_string(),
            None => {
                log::warn!("Unable to fetch entry path for entry: {:?}", entry);
                anyhow::bail!("Unable to fetch entry path");
            }
        };

        if ignore.contains(&entry_path) { continue; }

        if entry_type.is_file() {
            let entry_content = match read_to_string(&entry_path) {
                Ok(content) => content,
                Err(e) => {
                    if e.kind() == ErrorKind::InvalidData {
                        log::warn!("Unable to read file contents for file: {}: binary files are not supported yet.", entry_path);
                        continue;
                    };
                    log::error!("Unable to read file contents for file: {}: {}", entry_path, e.to_string());
                    anyhow::bail!("Unable to read file contents");
                },
            };

            files.insert(entry_path.clone(), entry_content);
        } else if entry_type.is_dir() {
            files.extend(build_tree(entry_path, ignore)?);
        };
    }

    Ok(files)
}

pub fn parse_ignore(root: String) -> anyhow::Result<Vec<String>> {
    let mut ignores = vec![];
    let content = read_to_string(&root)?;
    let lines = content.lines();

    for line in lines {
        if line.is_empty() {
            continue;
        };
        if line.starts_with('#') {
            continue;
        };

        let path: String = "./".to_string() + Path::new(line).to_str().unwrap();

        log::info!("Ignoring path: {} (root: {})", &path, &root);
        ignores.push(path);
    }

    Ok(ignores)
}
