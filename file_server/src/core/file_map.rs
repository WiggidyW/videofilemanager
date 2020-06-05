pub trait FileMap: Send + Sync + 'static {
    type Error: std::error::Error + 'static;
    fn get(&self, key: u32) -> Result<std::path::PathBuf, Self::Error>;
}

pub mod local_file_map {
    use super::FileMap;
    use std::{io, path::{Path, PathBuf}};

    pub struct LocalFileMap {
        base_path: PathBuf,
    }

    impl LocalFileMap {
        pub fn new(path: impl AsRef<Path>) -> Self {
            Self { base_path: path.as_ref().to_owned() }
        }
    }

    impl FileMap for LocalFileMap {
        type Error = io::Error;
        fn get(&self, key: u32) -> Result<PathBuf, Self::Error> {
            self.base_path.metadata()?;
            let mut path = self.base_path.clone();
            path.push(key.to_string());
            Ok(path)
        }
    }
}

// impl<T: FileMap + ?Sized> FileMap for Box<T> {
//     type Error = <T as FileMap>::Error;
//     fn get(&self, key: u32) -> Result<std::path::PathBuf, Self::Error> {
//         (**self).get(key)
//     }
// }