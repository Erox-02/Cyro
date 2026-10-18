use eframe::egui;
use image::DynamicImage;

pub struct Cyro {
    pub image: Option<DynamicImage>,
    pub texture: Option<egui::TextureHandle>,
    pub status: String,
}

impl Default for Cyro {
    fn default() -> Self {
        Self {
            image: None,
            texture: None,
            status: "open an image to begin.".into(),
        }
    }
}

impl Cyro {
    pub fn open_image(&mut self, ctx: &egui::Context) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .pick_file()
        else {
            return;
        };

        match image::open(&path) {
            Ok(image) => {
                let rgba = image.to_rgba8();
                let size = [rgba.width() as usize, rgba.height() as usize];

                let color_image =
                    egui::ColorImage::from_rgba_unmultiplied(
                        size,
                        rgba.as_raw(),
                    );

                self.texture = Some(ctx.load_texture(
                    "cyro-image",
                    color_image,
                    egui::TextureOptions::LINEAR,
                ));

                self.image = Some(image);
                self.status = format!("opened => {}", path.display());
            }
            Err(err) => {
                self.status = format!("could not open image => {err}");
            }
        }
    }

    pub fn export_image(&mut self) {
        let Some(image) = &self.image else {
            self.status = "no image to export.".into();
            return;
        };

        let Some(path) = rfd::FileDialog::new()
            .add_filter("png image", &["png"])
            .set_file_name("export.png")
            .save_file()
        else {
            return;
        };

        if path.exists() {
            self.status = "export cancelled => file already exists.".into();
            return;
        }

        match image.save_with_format(&path, image::ImageFormat::Png) {
            Ok(()) => {
                self.status = format!("exported => {}", path.display());
            }
            Err(err) => {
                self.status = format!("export failed => {err}");
            }
        }
    }
}

impl eframe::App for Cyro {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Open").clicked() {
                    self.open_image(ctx);
                }

                if ui.button("Export PNG").clicked() {
                    self.export_image();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                let available = ui.available_size();
                let image_size = texture.size_vec2();
                let scale = (available.x / image_size.x)
                    .min(available.y / image_size.y)
                    .min(1.0);

                ui.centered_and_justified(|ui| {
                    ui.add(
                        egui::Image::new(texture)
                            .fit_to_exact_size(image_size * scale),
                    );
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Cyro not cryo");
                });
            }
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.label(&self.status);
        });
    }
}
