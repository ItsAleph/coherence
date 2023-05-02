use crate::ast::*;

pub fn serialize_file(file: &CHRFile) -> String {
    let mut serialized = "".to_string();

    serialized.push_str(&serialize_header(&file.header));

    for patch in file.patches {
        serialized.push_str(serialize_patch(&patch));
    }

    serialized
}

pub fn serialize_header(header: &CHRHeader) -> String {
    let mut serialized = "".to_string();

    serialized.push_str(&serialize_author(&header.author));
    serialized.push_str(&format!("date: {}", header.date));
    serialized.push_str(&format!("description: {}", header.description));
}
