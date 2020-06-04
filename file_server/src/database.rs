pub trait Database: Send + Sync + 'static {
    type Error: std::error::Error + 'static;
    
    // Returns None if the file_id does not exist.
    fn get_aliases(
        &self,
        file_id: u32,
    ) -> Result<Option<Vec<String>>, Self::Error>;
    
    // Returns None if the alias does not exist.
    fn get_file_id(&self, alias: &str) -> Result<Option<u32>, Self::Error>;
    
    // Generates a new, unique file_id.
    fn create_file_id(&mut self) -> Result<u32, Self::Error>;

    // Returns None if the alias already exists, or if the file_id does not exist.
    fn create_alias(
        &mut self,
        alias: &str,
        file_id: u32,
    ) -> Result<Option<()>, Self::Error>;
    
    // Returns None if the alias does not exist.
    fn remove_alias(&mut self, alias: &str) -> Result<Option<()>, Self::Error>;

    fn list_aliases(&self) -> Result<Vec<Vec<String>>, Self::Error>;
}

impl<T: Database + ?Sized> Database for Box<T> {
    type Error = <T as Database>::Error;
    fn get_aliases(
        &self,
        file_id: u32,
    ) -> Result<Option<Vec<String>>, Self::Error>
    {
        (**self).get_aliases(file_id)
    }
    fn get_file_id(&self, alias: &str) -> Result<Option<u32>, Self::Error> {
        (**self).get_file_id(alias)
    }
    fn create_file_id(&mut self) -> Result<u32, Self::Error> {
        (**self).create_file_id()
    }
    fn create_alias(
        &mut self,
        alias: &str,
        file_id: u32,
    ) -> Result<Option<()>, Self::Error>
    {
        (**self).create_alias(alias, file_id)
    }
    fn remove_alias(&mut self, alias: &str) -> Result<Option<()>, Self::Error> {
        (**self).remove_alias(alias)
    }
    fn list_aliases(&self) -> Result<Vec<Vec<String>>, Self::Error> {
        (**self).list_aliases()
    }
}

mod sqlite_database {
    use std::path::Path;
    use super::Database;
    use rusqlite::{Connection};

    pub struct SqliteDatabase {
        conn: Connection,
    }

    impl SqliteDatabase {
        pub fn new(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
            let self_ = Self {
                conn: Connection::open(path)?
            };
            self_.initialize()?;
            Ok(self_)
        }
        fn initialize(&self) -> Result<(), rusqlite::Error> {
            self.conn.execute_batch(
                "
                BEGIN;
                CREATE TABLE IF NOT EXISTS file_ids (
                    id INTEGER PRIMARY KEY AUTOINCREMENT
                );
                CREATE TABLE IF NOT EXISTS aliases (
                    id TEXT PRIMARY KEY,
                    file_id INTEGER NOT NULL,
                    FOREIGN KEY (file_id) REFERENCES file_ids (id)
                );
                COMMIT;
                "
            )
        }
    }
}