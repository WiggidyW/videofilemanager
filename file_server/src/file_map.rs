pub trait FileMap {
    type Error: std::error::Error + 'static;
    fn get(&self, key: u32) -> Result<std::path::PathBuf, Self::Error>;
}