pub trait FileMap: Send + Sync + 'static {
    type Error: std::error::Error + 'static;
    fn get(&self, key: u32) -> Result<std::path::PathBuf, Self::Error>;
}