use std::{fs::create_dir_all, io::ErrorKind, path::PathBuf};

use serde::{de::DeserializeOwned, Serialize};

pub trait File: Serialize + DeserializeOwned + Default {
    fn path() -> PathBuf;

    fn load() -> Self {
        let path = Self::path();
        let file = if let Ok(file) = std::fs::File::open(path) {
            file
        } else {
            return Self::default();
        };

        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }

    fn save(&self) {
        let path = Self::path();
        let file = match std::fs::File::create(&path) {
            Ok(file) => file,
            Err(e) if e.kind() == ErrorKind::NotFound => self.create_file(),
            Err(e) => panic!("{:?}", e),
        };
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self).unwrap();
    }

    fn create_file(&self) -> std::fs::File {
        let path = Self::path();
        create_dir_all(&path.parent().unwrap()).unwrap();
        let file = std::fs::File::options()
            .create(true)
            .write(true)
            .open(path)
            .unwrap();
        file
    }
}
