#![allow(unused)]

use library::Library;

pub struct FsAdaptor {}

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
