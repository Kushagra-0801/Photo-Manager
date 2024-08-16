#![allow(unused)]

use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use library::Library;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Index {}

pub struct FsAdaptor {
    basepath: PathBuf,
    index: Index,
}

impl FsAdaptor {
    pub fn new(basepath: impl Into<PathBuf>) -> Result<Self, String> {
        let basepath: PathBuf = basepath.into();
        if !basepath.try_exists().map_err(|e| e.to_string())? {
            fs::create_dir_all(&basepath).map_err(|e| e.to_string())?
        }
        if !basepath.is_dir() {
            return Err(format!("{basepath:?} is not a directory"));
        }
        let index_file = basepath.join("index.json");
        let index_file = File::open(index_file).map_err(|e| e.to_string())?;
        let reader = BufReader::new(index_file);
        let index: Index = serde_json::from_reader(reader).map_err(|e| e.to_string())?;

        Ok(Self { basepath, index })
    }
}

impl Library for FsAdaptor {
    async fn search_images(
        &self,
        query: library::Query,
    ) -> library::Result<Vec<library::ImageMetadata>> {
        todo!()
    }

    async fn add_image(
        &mut self,
        image: library::ImageData,
    ) -> library::Result<library::ImageMetadata> {
        todo!()
    }

    async fn find_image(
        &self,
        image_id: library::ImageId,
    ) -> library::Result<library::ImageMetadata> {
        todo!()
    }

    async fn update_image_tags(
        &mut self,
        image_id: library::ImageId,
        new_tags: Vec<String>,
    ) -> library::Result<()> {
        todo!()
    }

    async fn delete_image(&mut self, image_id: library::ImageId) -> library::Result<()> {
        todo!()
    }

    async fn add_tag(&mut self, tag: String) -> library::Result<library::Tag> {
        todo!()
    }

    async fn get_tag_list(&self) -> library::Result<library::TagGraph> {
        todo!()
    }

    async fn remove_tag(&mut self, tag: library::Tag) -> library::Result<()> {
        todo!()
    }

    async fn get_image_data(
        &self,
        image_id: library::ImageId,
        format: String,
    ) -> library::Result<Vec<u8>> {
        todo!()
    }
}
