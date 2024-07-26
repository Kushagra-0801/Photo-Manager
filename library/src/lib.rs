#![allow(unused)]

use std::error::Error;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct ImageId(u32);

pub type ImageId = u32;

/// Example:
/// let image_metadata = ImageMetadata {
///     id: ImageId(42),
///     available_formats: vec!["jpg", "png", "webp"],
///     tags: vec!["abc", "def"]
/// }
pub struct ImageMetadata {
    pub id: ImageId,
    pub available_formats: Vec<String>,
    pub tags: Vec<Tag>,
}

pub struct ImageData {
    pub metadata: ImageMetadata,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(String);

pub struct TagGraph {}

/// A list of tags, to be searched by AND
pub struct Query {
    // list of tags
    // - Tag name or Tag Id?
    // - Can be searched by AND or OR
    // Should I do a whole INCLUSION / EXCLUSION / multiple AND conditions joined by OR? Let's do it later
    pub tags: Vec<Tag>,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[allow(async_fn_in_trait)]
pub trait Library {
    async fn search_images(&self, query: Query) -> Result<Vec<ImageMetadata>>;

    async fn add_image(&mut self, image: ImageData) -> Result<ImageMetadata>;
    async fn find_image(&self, image_id: ImageId) -> Result<ImageMetadata>;
    async fn update_image_tags(&mut self, image_id: ImageId, new_tags: Vec<String>) -> Result<()>;
    async fn delete_image(&mut self, image_id: ImageId) -> Result<()>;

    async fn add_tag(&mut self, tag: String) -> Result<Tag>;
    async fn get_tag_list(&self) -> Result<TagGraph>;
    async fn remove_tag(&mut self, tag: Tag) -> Result<()>;

    async fn get_image_data(&self, image_id: ImageId, format: String) -> Result<Vec<u8>>;
}

impl Library for () {
    async fn search_images(&self, query: Query) -> Result<Vec<ImageMetadata>> {
        todo!()
    }

    async fn add_image(&mut self, image: ImageData) -> Result<ImageMetadata> {
        todo!()
    }

    async fn find_image(&self, image_id: ImageId) -> Result<ImageMetadata> {
        todo!()
    }

    async fn update_image_tags(&mut self, image_id: ImageId, new_tags: Vec<String>) -> Result<()> {
        todo!()
    }

    async fn delete_image(&mut self, image_id: ImageId) -> Result<()> {
        todo!()
    }

    async fn add_tag(&mut self, tag: String) -> Result<Tag> {
        todo!()
    }

    async fn get_tag_list(&self) -> Result<TagGraph> {
        todo!()
    }

    async fn remove_tag(&mut self, tag: Tag) -> Result<()> {
        todo!()
    }

    async fn get_image_data(&self, image_id: ImageId, format: String) -> Result<Vec<u8>> {
        todo!()
    }
}

pub fn format_to_mimetype(format: &str) -> &'static str {
    todo!()
}
