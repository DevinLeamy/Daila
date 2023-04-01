use std::path::PathBuf;

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
        let file = std::fs::File::create(path).unwrap();
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self).unwrap();
    }
}
