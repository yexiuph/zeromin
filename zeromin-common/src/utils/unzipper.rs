use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use zip::read::ZipArchive;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FileStream {
    pub file_name: String,
    pub file_content: Arc<Vec<u8>>,
}

#[repr(C)]
pub struct CUnzipper {
    pub files: Arc<Mutex<Vec<FileStream>>>,
}

impl CUnzipper {
    pub fn new() -> Self {
        Self {
            files: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn load_rcc(&self, zip_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let mut content = Vec::new();
            file.read_to_end(&mut content)?;

            let file_info = FileStream {
                file_name: file.name().to_string(),
                file_content: Arc::new(content),
            };
            self.files.lock().unwrap().push(file_info);
        }

        Ok(())
    }
}
