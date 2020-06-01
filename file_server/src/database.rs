pub trait Database: Send + Sync + 'static {
    type Error: std::error::Error + 'static;
    
    // Returns None if the file_id does not exist.
    fn get_aliases(&self, file_id: u32) -> Result<
        Option<Vec<String>>,
        Self::Error,
    >;
    
    // Returns None if the alias does not exist.
    fn get_file_id(&self, alias: &str) -> Result<
        Option<u32>,
        Self::Error,
    >;
    
    // Generates a new, unique file_id.
    fn create_file_id(&self) -> Result<
        u32,
        Self::Error,
    >;

    // Returns None if the alias already exists, or if the file_id does not exist.
    fn create_alias(&self, alias: &str, file_id: u32) -> Result<
        Option<()>,
        Self::Error,
    >;
    
    // Returns None if the alias does not exist.
    fn remove_alias(&self, alias: &str) -> Result<
        Option<()>,
        Self::Error,
    >;

    fn list_aliases(&self) -> Result<
        Vec<Vec<String>>,
        Self::Error,
    >;
}