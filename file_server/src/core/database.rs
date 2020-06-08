pub trait Database: Send + Sync + 'static {
    type Error: std::error::Error + 'static;

    fn id_exists(&self, file_id: u32) -> Result<bool, Self::Error>;

    fn list_ids(&self) -> Result<Vec<u32>, Self::Error>;
    
    // Generates a new, unique file_id.
    fn create_id(&self) -> Result<u32, Self::Error>;
    
    // Returns None if the alias does not exist.
    fn get_id(&self, alias: &str) -> Result<Option<u32>, Self::Error>;

    // Returns None if any aliases exist, or file_id does not exist.
    fn create_aliases(
        &self,
        aliases: Vec<String>,
        file_id: u32,
    ) -> Result<Option<()>, Self::Error>;
    
    // Returns None if any do not exist.
    fn remove_aliases(
        &self,
        aliases: Vec<String>,
        file_id: u32,
    ) -> Result<Option<()>, Self::Error>;
    
    // Returns None if there are no aliases.
    fn get_aliases(
        &self,
        file_id: u32,
    ) -> Result<Option<Vec<String>>, Self::Error>;
}

pub mod sqlite_database {
    use std::{path::Path, sync::Mutex, convert::TryFrom};
    use super::Database;
    use rusqlite::{Connection, params, OptionalExtension};

    #[derive(Debug)]
    pub struct SqliteDatabase {
        conn: Mutex<Connection>,
    }

    impl SqliteDatabase {
        pub fn new(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
            let self_ = Self {
                conn: Mutex::new(
                    Connection::open(path)?
                ),
            };
            self_.initialize()?;
            Ok(self_)
        }
        fn initialize(&self) -> Result<(), rusqlite::Error> {
            self.conn.lock().unwrap().execute_batch(
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

        // https://github.com/rusqlite/rusqlite/blob/33d40aac107f1887803a6c9dff425c8d1fbb89e7/libsqlite3-sys/bindgen-bindings/bindgen_3.6.23.rs
        // https://docs.rs/libsqlite3-sys/0.18.0/src/libsqlite3_sys/error.rs.html#66-98
        fn check_constraint<T>(
            e: rusqlite::Error,
        ) -> Result<Option<T>, rusqlite::Error>
        {
            if let rusqlite::Error::SqliteFailure(sys_err, _) = e {
                if sys_err.extended_code & 0xff == 19 {
                    return Ok(None);
                }
            }
            Err(e)
        }
    }

    impl Database for SqliteDatabase {
        type Error = rusqlite::Error;
        fn id_exists(&self, file_id: u32) -> Result<bool, Self::Error> {
            self.conn.lock().unwrap()
                .query_row(
                    "SELECT EXISTS (SELECT 1 FROM file_ids WHERE id = (?))",
                    params![file_id],
                    |row| row.get::<usize, bool>(0),
                )
        }
        fn list_ids(&self) -> Result<Vec<u32>, Self::Error> {
            self.conn.lock().unwrap()
                .prepare("SELECT id FROM file_ids")?
                .query(params![])
                .optional()?
                .map(|iter| iter.mapped(|row| row.get::<usize, u32>(0)))
                .map(|iter| iter.collect())
                .unwrap_or(Ok(Vec::new()))
        }
        fn create_id(&self) -> Result<u32, Self::Error> {
            let conn = self.conn.lock().unwrap();
            conn.execute("INSERT INTO file_ids DEFAULT VALUES", params![])?;
            Ok(u32::try_from(conn.last_insert_rowid()).unwrap_or(0))
        }
        fn get_id(&self, alias: &str) -> Result<Option<u32>, Self::Error> {
            self.conn.lock().unwrap()
                .query_row(
                    "SELECT file_id FROM aliases WHERE id = (?)",
                    params![alias],
                    |row| row.get::<usize, u32>(0)
                ).optional()
        }
        fn create_aliases(
            &self,
            aliases: Vec<String>,
            file_id: u32,
        ) -> Result<Option<()>, Self::Error>
        {
            let mut conn = self.conn.lock().unwrap();
            let transaction = conn.transaction()?;
            for alias in aliases.into_iter() {
                if let Err(e) = transaction.execute(
                    "INSERT INTO aliases VALUES (?, ?)",
                    params![alias, file_id],
                ) {
                    return Self::check_constraint(e);
                }
            }
            transaction.commit().map(|_| Some(()))
        }
        fn remove_aliases(
            &self,
            aliases: Vec<String>,
            file_id: u32,
        ) -> Result<Option<()>, Self::Error>
        {
            let mut conn = self.conn.lock().unwrap();
            let transaction = conn.transaction()?;
            for alias in aliases.into_iter() {
                match transaction.execute(
                    "DELETE FROM aliases WHERE id = (?) AND file_id = (?)",
                    params![alias, file_id],
                ) {
                    Ok(0) => return Ok(None),
                    Ok(_) => (),
                    Err(e) => return Self::check_constraint(e),
                };
            }
            transaction.commit().map(|_| Some(()))
        }
        fn get_aliases(
            &self,
            file_id: u32,
        ) -> Result<Option<Vec<String>>, Self::Error>
        {
            self.conn.lock().unwrap()
                .prepare("SELECT id FROM aliases WHERE file_id = (?)")?
                .query(params![file_id])
                .optional()?
                .map(|iter| iter.mapped(|row| row.get::<usize, String>(0)))
                .map(|iter| iter.collect())
                .transpose()
        }
    }
}