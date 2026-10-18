use image::DynamicImage;
use std::path::Path;

pub fn load(path: &Path) -> image::ImageResult<DynamicImage> {
    image::open(path)
}

pub fn save(
    image: &DynamicImage,
    path: &Path,
) -> image::ImageResult<()> {
    image.save(path)
}
