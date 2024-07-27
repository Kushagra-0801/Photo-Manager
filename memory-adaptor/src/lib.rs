use library::Library;

const _IMAGE_ID_IS_AN_INTEGER: library::ImageId = 0;

struct Image {
    id: library::ImageId,
    tags: Vec<library::Tag>,
    data: Vec<ImageData>,
}

struct ImageData {
    format: String,
    data: Vec<u8>,
}

impl From<Image> for library::ImageMetadata {
    fn from(value: Image) -> Self {
        Self {
            id: value.id,
            available_formats: value.data.iter().map(|d| d.format.clone()).collect(),
            tags: value.tags.clone(),
        }
    }
}

impl From<&Image> for library::ImageMetadata {
    fn from(value: &Image) -> Self {
        Self {
            id: value.id,
            available_formats: value.data.iter().map(|d| d.format.clone()).collect(),
            tags: value.tags.clone(),
        }
    }
}

#[derive(Default)]
pub struct MemoryAdaptor {
    images: Vec<Image>,
}

impl MemoryAdaptor {
    // fn convert
    fn get_next_id(&self) -> library::ImageId {
        (self.images.len() + 1) as _
    }
}

#[allow(unused)]
impl Library for MemoryAdaptor {
    async fn search_images(
        &self,
        query: library::Query,
    ) -> library::Result<Vec<library::ImageMetadata>> {
        Ok(self
            .images
            .iter()
            .filter(|(i)| query.tags.iter().all(|t| i.tags.contains(t)))
            .map(Into::into)
            .collect())
    }

    async fn add_image(
        &mut self,
        mut image: library::ImageData,
    ) -> library::Result<library::ImageMetadata> {
        assert_eq!(image.metadata.id, 0);
        assert_eq!(image.metadata.available_formats.len(), 1);
        let image = Image {
            id: self.get_next_id(),
            tags: image.metadata.tags,
            data: vec![ImageData {
                format: image.metadata.available_formats.pop().unwrap(),
                data: image.data,
            }],
        };
        self.images.push(image);
        let image_metadata = self.images.last().unwrap().into();
        Ok(image_metadata)
    }

    async fn find_image(
        &self,
        image_id: library::ImageId,
    ) -> library::Result<library::ImageMetadata> {
        Ok((&self.images[(image_id - 1) as usize]).into())
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
        Ok(self.images[(image_id - 1) as usize]
            .data
            .iter()
            .find(|d| d.format == format)
            .map(|d| d.data.clone())
            .unwrap())
    }
}
