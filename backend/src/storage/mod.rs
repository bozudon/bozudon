use mime::Mime;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::path::PathBuf;
use uuid::Uuid;

pub struct SavedMedia {
    pub media_type: String,
    pub key: String,
    pub thumbnail_key: String,
}

#[derive(Clone)]
pub struct LocalStorage {
    base_dir: PathBuf,
}

impl LocalStorage {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn save_file<R>(&self, stream: &mut R, content_type: Mime) -> Result<SavedMedia, Error>
    where
        R: Read,
    {
        log::info!("{}", content_type.essence_str());
        let ext = match content_type.essence_str() {
            "image/png" => String::from("png"),
            "image/jpeg" => String::from("jpg"),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "only supported format is image/png",
                ))
            }
        };

        let id = Uuid::new_v4().to_string();
        let filename = format!("{}.{}", id, ext);

        let mut out_file = File::create(self.base_dir.clone().join(&filename))?;

        std::io::copy(stream, &mut out_file)?;

        Ok(SavedMedia {
            media_type: String::from("image"),
            key: filename.clone(),
            thumbnail_key: filename,
        })
    }

    pub fn get_file(&self, key: &str) -> Result<File, Error> {
        File::open(self.base_dir.clone().join(key))
    }
}
