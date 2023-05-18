use rusqlite::{Connection, Result, OpenFlags};

pub struct Database {
    connection: Connection,
}


trait DatabaseController {
    pub fn init(path_to_file: &str) -> Self {
        let connection = Connection::open(&path_to_file).unwrap();

        println!("{}", connection.is_autocommit());
        Self { connection }
    };

    pub fn add_file(path_to_file: &str);
    pub fn add_changes(path_to_file: &str);
    pub fn remove_file(path_to_file: &str);
    pub fn remove_changes(path_to_file: &str);
    pub fn add_commit(path_to_file: &str);
}


impl DatabaseController for Database {
    pub fn add_file(&self, path_to_file: &str) {
        todo!();
    };
    pub fn add_changes(&self, path_to_file: &str) {
        todo!();
    };
    pub fn remove_file(&self, path_to_file: &str) {
        todo!();
    };
    pub fn remove_changes(&self, path_to_file: &str) {
        todo!();
    };
    pub fn add_commit(&self, path_to_file: &str) {
        todo!();
    };
}