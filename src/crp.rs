use image::DynamicImage;

#[derive(Debug, Clone, Copy, Default)]
pub struct Crp {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Crp {
    pub fn crp_img(
        &self,
        image: &DynamicImage,
    ) -> Result<DynamicImage, String> {
        let image_width = image.width();
        let image_height = image.height();

        if self.width == 0 || self.height == 0 {
            return Err("Crop area cannot be empty.".into());
        }

        let right = self.x
            .checked_add(self.width)
            .ok_or("Crop coordinates overflow.")?;

        let bottom = self.y
            .checked_add(self.height)
            .ok_or("Crop coordinates overflow.")?;

        if right > image_width || bottom > image_height {
            return Err("Crop area exceeds image boundaries.".into());
        }

        Ok(image.crop_imm(
            self.x,
            self.y,
            self.width,
            self.height,
        ))
    }
}
