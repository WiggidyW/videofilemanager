pub trait FileMap: Send + Sync + 'static {
    type Error: std::error::Error + 'static;
    fn get(&self, key: u32) -> Result<std::path::PathBuf, Self::Error>;
}

impl<T: FileMap + ?Sized> FileMap for Box<T> {
    type Error = <T as FileMap>::Error;
    fn get(&self, key: u32) -> Result<std::path::PathBuf, Self::Error> {
        (**self).get(key)
    }
}