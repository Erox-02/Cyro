use eframe::egui;
use image::DynamicImage;

use crate::crp::Crp;

pub struct Cyro {
    pub image: Option<DynamicImage>,
    pub texture: Option<egui::TextureHandle>,
    pub status: String,
    pub crop: Crp,
    crop_start: Option<egui::Pos2>,
    crop_end: Option<egui::Pos2>,
}

impl Default for Cyro {
    fn default() -> Self {
        Self {
            image: None,
            texture: None,
            status: "open an image to begin.".into(),
            crop: Crp::default(),
            crop_start: None,
            crop_end: None,
        }
    }
}

impl Cyro {
    fn refresh_texture(&mut self, ctx: &egui::Context) {
        let Some(image) = &self.image else {
            self.texture = None;
            return;
        };

        let rgba = image.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];

        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            size,
            rgba.as_raw(),
        );

        self.texture = Some(ctx.load_texture(
            "cyro-image",
            color_image,
            egui::TextureOptions::LINEAR,
        ));
    }

    pub fn open_image(&mut self, ctx: &egui::Context) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .pick_file()
        else {
            return;
        };

        match image::open(&path) {
            Ok(image) => {
                self.image = Some(image);
                self.crop = Crp::default();
                self.crop_start = None;
                self.crop_end = None;
                self.refresh_texture(ctx);

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
            .set_file_name("export png")
            .save_file()
        else {
            return;
        };

        if path.exists() {
            self.status =
                "export cancelled => file already exists.".into();
            return;
        }

        match image.save_with_format(
            &path,
            image::ImageFormat::Png,
        ) {
            Ok(()) => {
                self.status = format!("exported => {}", path.display());
            }
            Err(err) => {
                self.status = format!("export failed => {err}");
            }
        }
    }

    pub fn apply_crop(&mut self, ctx: &egui::Context) {
        let Some(image) = &self.image else {
            self.status = "no image loaded.".into();
            return;
        };

        match self.crop.crp_img(image) {
            Ok(cropped) => {
                self.image = Some(cropped);
                self.crop = Crp::default();
                self.crop_start = None;
                self.crop_end = None;

                self.refresh_texture(ctx);
                self.status = "crop applied.".into();
            }
            Err(err) => {
                self.status = format!("crop failed => {err}");
            }
        }
    }

    fn cancel_crop(&mut self) {
        self.crop = Crp::default();
        self.crop_start = None;
        self.crop_end = None;
    }

    fn image_position(
        pos: egui::Pos2,
        image_rect: egui::Rect,
        image_width: u32,
        image_height: u32,
    ) -> egui::Pos2 {
        egui::pos2(
            ((pos.x - image_rect.left()) / image_rect.width())
                .clamp(0.0, 1.0)
                * image_width as f32,
            ((pos.y - image_rect.top()) / image_rect.height())
                .clamp(0.0, 1.0)
                * image_height as f32,
        )
    }

    fn update_crop_rect(
        &mut self,
        image_width: u32,
        image_height: u32,
    ) {
        let (Some(start), Some(end)) =
            (self.crop_start, self.crop_end)
        else {
            return;
        };

        let x = start.x.min(end.x).floor().max(0.0) as u32;
        let y = start.y.min(end.y).floor().max(0.0) as u32;

        let right = start.x.max(end.x).ceil()
            .clamp(0.0, image_width as f32) as u32;

        let bottom = start.y.max(end.y).ceil()
            .clamp(0.0, image_height as f32) as u32;

        self.crop = Crp {
            x,
            y,
            width: right.saturating_sub(x),
            height: bottom.saturating_sub(y),
        };
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

                if ui.button("Apply").clicked() {
                    self.apply_crop(ctx);
                }

                if ui.button("Cancel").clicked() {
                    self.cancel_crop();
                    self.status = "crop selection cancelled.".into();
                }

                if ui.button("esport png").clicked() {
                    self.export_image();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let available = ui.available_size();

            let (canvas_rect, response) = ui.allocate_exact_size(
                available,
                egui::Sense::click_and_drag(),
            );

            let painter = ui.painter_at(canvas_rect);

            painter.rect_filled(
                canvas_rect,
                0.0,
                egui::Color32::from_gray(30),
            );

            if let (Some(image), Some(texture)) =
                (&self.image, &self.texture)
            {
                let image_width = image.width();
                let image_height = image.height();

                let image_size = texture.size_vec2();

                let scale = (canvas_rect.width() / image_size.x)
                    .min(canvas_rect.height() / image_size.y)
                    .min(1.0);

                let display_size = image_size * scale;

                let image_rect = egui::Rect::from_center_size(
                    canvas_rect.center(),
                    display_size,
                );

                painter.image(
                    texture.id(),
                    image_rect,
                    egui::Rect::from_min_max(
                        egui::pos2(0.0, 0.0),
                        egui::pos2(1.0, 1.0),
                    ),
                    egui::Color32::WHITE,
                );

                if response.drag_started() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        if image_rect.contains(pos) {
                            let pixel = Self::image_position(
                                pos,
                                image_rect,
                                image_width,
                                image_height,
                            );

                            self.crop_start = Some(pixel);
                            self.crop_end = Some(pixel);
                        }
                    }
                }

                if response.dragged() && self.crop_start.is_some() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        self.crop_end = Some(Self::image_position(
                            pos,
                            image_rect,
                            image_width,
                            image_height,
                        ));
                    }
                }

                self.update_crop_rect(image_width, image_height);

                if let (Some(start), Some(end)) =
                    (self.crop_start, self.crop_end)
                {
                    let to_screen = |p: egui::Pos2| {
                        egui::pos2(
                            image_rect.left()
                                + p.x / image_width as f32
                                    * image_rect.width(),
                            image_rect.top()
                                + p.y / image_height as f32
                                    * image_rect.height(),
                        )
                    };

                    let selection = egui::Rect::from_two_pos(
                        to_screen(start),
                        to_screen(end),
                    );

                    painter.rect_filled(
                        selection,
                        0.0,
                        egui::Color32::from_white_alpha(35),
                    );

                    painter.rect_stroke(
                        selection,
                        0.0,
                        egui::Stroke::new(
                            2.0,
                            egui::Color32::LIGHT_GREEN,
                        ),
                        egui::StrokeKind::Inside,
                    );
                }
            } else {
                painter.text(
                    canvas_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "CYRO — not cryo\n\nOpen an image to begin",
                    egui::FontId::proportional(24.0),
                    egui::Color32::LIGHT_GRAY,
                );
            }
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status);

                if self.image.is_some() {
                    ui.separator();
                    ui.label(format!(
                        "Crop: {} × {}",
                        self.crop.width,
                        self.crop.height,
                    ));
                }
            });
        });
    }
}
