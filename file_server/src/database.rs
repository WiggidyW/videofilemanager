pub trait Database {
    type Error: std::error::Error + 'static;
    
    // Returns None if the file_id does not exist.
    fn get_aliases(&mut self, file_id: u32) -> Result<
        Option<Vec<String>>,
        Self::Error,
    >;
    
    // Returns None if the alias does not exist.
    fn get_file_id(&mut self, alias: &str) -> Result<
        Option<u32>,
        Self::Error,
    >;
    
    // Generates a new, unique file_id.
    fn create_file_id(&mut self) -> Result<
        u32,
        Self::Error,
    >;

    // // Starts a transaction.
    // fn start_transaction(&mut self);

    // // Discards the transaction.
    // fn discard_transaction(&mut self);

    // // Commits all changes to the database.
    // fn commit_transaction(&mut self) -> Result<(), Self::Error>;

    // Returns None if the alias already exists, or if the file_id does not exist.
    fn create_alias(&mut self, alias: &str, file_id: u32) -> Result<
        Option<()>,
        Self::Error,
    >;
    
    // Returns None if the file_id does not exist.
    fn remove_file_id(&mut self, file_id: u32) -> Result<
        Option<()>,
        Self::Error,
    >;
    
    // Returns None if the alias does not exist.
    fn remove_alias(&mut self, alias: &str) -> Result<
        Option<()>,
        Self::Error,
    >;
}