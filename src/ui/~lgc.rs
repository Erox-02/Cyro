use eframe::egui;
use image::DynamicImage;

use crate::crp::Crp;
use super::bar::{self, BarAction};
use super::bkgnd;

#[derive(Clone, Copy)]
enum Dragmd {
    Move,
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    New,
}

pub struct Cyro {
    pub image: Option<DynamicImage>,
    pub texture: Option<egui::TextureHandle>,
    pub status: String,

    pub crop: Crp,
    crop_mode: bool,
    drag_mode: Option<Dragmd>,
    drag_anchor: egui::Pos2,
    drag_initial: Crp,
}

impl Default for Cyro {
    fn default() -> Self {
        Self {
            image: None,
            texture: None,
            status: "Open an image to begin.".into(),
            crop: Crp::default(),
            crop_mode: false,
            drag_mode: None,
            drag_anchor: egui::Pos2::ZERO,
            drag_initial: Crp::default(),
        }
    }
}

impl Cyro {
    pub fn show_ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let action = bar::show(ui, self.crop_mode);
            self.handle_action(action, ctx);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.image.is_none() {
                ui.centered_and_justified(|ui| {
                    ui.label("CYRO — not cryo\n\nOpen an image to begin.");
                });
                return;
            }

            let (width, height) = self
                .image
                .as_ref()
                .map(|img| (img.width(), img.height()))
                .unwrap_or((0, 0));

            let image_rect = bkgnd::show(
                ui,
                self.texture.as_ref(),
                self.crop_mode,
                self.crop,
                width,
                height,
            );

            let Some(image_rect) = image_rect else {
                return;
            };

            if !self.crop_mode {
                return;
            }

            let response = ui.interact(
                image_rect,
                egui::Id::new("cyro_crop_interaction"),
                egui::Sense::click_and_drag(),
            );

            if response.drag_started() {
                if let Some(pointer) = response.interact_pointer_pos() {
                    let crop_rect =
                        self.crop_screen_rect(image_rect, width, height);

                    self.drag_mode = Some(if crop_rect.contains(pointer) {
                        Self::hit_test(pointer, crop_rect)
                    } else {
                        Dragmd::New
                    });

                    self.drag_anchor =
                        Self::screen_to_pixel(pointer, image_rect, width, height);

                    self.drag_initial = self.crop;
                }
            }

            if response.dragged() {
                if let Some(pointer) = response.interact_pointer_pos() {
                    self.update_drag(pointer, image_rect, width, height);
                }
            }

            if response.drag_stopped() {
                self.drag_mode = None;
            }
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status);

                if self.crop_mode {
                    ui.separator();
                    ui.label(format!(
                        "{} × {} px",
                        self.crop.width, self.crop.height
                    ));
                }
            });
        });
    }

    fn handle_action(&mut self, action: BarAction, ctx: &egui::Context) {
        match action {
            BarAction::None => {}
            BarAction::Open => self.open_image(ctx),
            BarAction::EnterCrop => self.start_crop(),
            BarAction::ApplyCrop => self.apply_crop(ctx),
            BarAction::CancelCrop => self.cancel_crop(),
            BarAction::Export => self.export_image(),
        }
    }

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

    fn open_image(&mut self, ctx: &egui::Context) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .pick_file()
        else {
            return;
        };

        match image::open(&path) {
            Ok(image) => {
                self.image = Some(image);
                self.refresh_texture(ctx);
                self.crop = Crp::default();
                self.crop_mode = false;
                self.drag_mode = None;
                self.status = format!("opened => {}", path.display());
            }
            Err(err) => {
                self.status = format!("Could not open image => {err}");
            }
        }
    }

    fn export_image(&mut self) {
        let Some(image) = &self.image else {
            self.status = "No image to export.".into();
            return;
        };

        let Some(path) = rfd::FileDialog::new()
            .add_filter("PNG image", &["png"])
            .set_file_name("export.png")
            .save_file()
        else {
            return;
        };

        if path.exists() {
            self.status =
                "export cancelled => file exists.".into();
            return;
        }

        match image.save_with_format(&path, image::ImageFormat::Png) {
            Ok(()) => self.status = format!("Exported => {}", path.display()),
            Err(err) => self.status = format!("Export failed => {err}"),
        }
    }

    fn start_crop(&mut self) {
        let Some(image) = &self.image else {
            self.status = "open an image first.".into();
            return;
        };

        let width = image.width();
        let height = image.height();

        let margin_x = (width / 20).min(width.saturating_sub(1));
        let margin_y = (height / 20).min(height.saturating_sub(1));

        self.crop = Crp {
            x: margin_x,
            y: margin_y,
            width: width.saturating_sub(2 * margin_x).max(1),
            height: height.saturating_sub(2 * margin_y).max(1),
        };

        self.crop_mode = true;
        self.drag_mode = None;
        self.status = "drag the white handles to crop.".into();
    }

    fn apply_crop(&mut self, ctx: &egui::Context) {
        let result = match &self.image {
            Some(image) => self.crop.crp_img(image),
            None => {
                self.status = "no image loaded.".into();
                return;
            }
        };

        match result {
            Ok(cropped) => {
                let (width, height) = (cropped.width(), cropped.height());
                self.image = Some(cropped);
                self.crop_mode = false;
                self.drag_mode = None;
                self.crop = Crp::default();
                self.refresh_texture(ctx);
                self.status = format!("Cropped => {width} × {height} pixels");
            }
            Err(err) => self.status = format!("Crop failed => {err}"),
        }
    }

    fn cancel_crop(&mut self) {
        self.crop_mode = false;
        self.drag_mode = None;
        self.crop = Crp::default();
        self.status = "Crop cancelled.".into();
    }

    fn crop_screen_rect(
        &self,
        image_rect: egui::Rect,
        width: u32,
        height: u32,
    ) -> egui::Rect {
        egui::Rect::from_min_max(
            Self::pixel_to_screen(
                egui::pos2(self.crop.x as f32, self.crop.y as f32),
                image_rect, width, height,
            ),
            Self::pixel_to_screen(
                egui::pos2(
                    (self.crop.x + self.crop.width) as f32,
                    (self.crop.y + self.crop.height) as f32,
                ),
                image_rect, width, height,
            ),
        )
    }

    fn pixel_to_screen(
        p: egui::Pos2,
        rect: egui::Rect,
        width: u32,
        height: u32,
    ) -> egui::Pos2 {
        egui::pos2(
            rect.left() + p.x / width as f32 * rect.width(),
            rect.top() + p.y / height as f32 * rect.height(),
        )
    }

    fn screen_to_pixel(
        p: egui::Pos2,
        rect: egui::Rect,
        width: u32,
        height: u32,
    ) -> egui::Pos2 {
        egui::pos2(
            ((p.x - rect.left()) / rect.width()).clamp(0.0, 1.0) * width as f32,
            ((p.y - rect.top()) / rect.height()).clamp(0.0, 1.0) * height as f32,
        )
    }

    fn hit_test(pos: egui::Pos2, rect: egui::Rect) -> Dragmd {
        let near = |p: egui::Pos2| (pos - p).length() < 14.0;

        if near(rect.left_top()) { Dragmd::TopLeft }
        else if near(rect.right_top()) { Dragmd::TopRight }
        else if near(rect.left_bottom()) { Dragmd::BottomLeft }
        else if near(rect.right_bottom()) { Dragmd::BottomRight }
        else if near(egui::pos2(rect.center().x, rect.top())) { Dragmd::Top }
        else if near(egui::pos2(rect.center().x, rect.bottom())) { Dragmd::Bottom }
        else if near(egui::pos2(rect.left(), rect.center().y)) { Dragmd::Left }
        else if near(egui::pos2(rect.right(), rect.center().y)) { Dragmd::Right }
        else { Dragmd::Move }
    }

    fn update_drag(
        &mut self,
        pointer: egui::Pos2,
        rect: egui::Rect,
        width: u32,
        height: u32,
    ) {
        let point = Self::screen_to_pixel(pointer, rect, width, height);
        let dx = point.x - self.drag_anchor.x;
        let dy = point.y - self.drag_anchor.y;
        let old = self.drag_initial;

        let mut left = old.x as f32;
        let mut top = old.y as f32;
        let mut right = (old.x + old.width) as f32;
        let mut bottom = (old.y + old.height) as f32;

        match self.drag_mode {
            Some(Dragmd::New) => {
                left = self.drag_anchor.x.min(point.x);
                top = self.drag_anchor.y.min(point.y);
                right = self.drag_anchor.x.max(point.x);
                bottom = self.drag_anchor.y.max(point.y);
            }
            Some(Dragmd::Move) => {
                let w = right - left;
                let h = bottom - top;
                left = (left + dx).clamp(0.0, width as f32 - w);
                top = (top + dy).clamp(0.0, height as f32 - h);
                right = left + w;
                bottom = top + h;
            }
            Some(Dragmd::Left | Dragmd::TopLeft | Dragmd::BottomLeft) => {
                left = (old.x as f32 + dx).clamp(0.0, right - 1.0);
            }
            _ => {}
        }

        match self.drag_mode {
            Some(Dragmd::Right | Dragmd::TopRight | Dragmd::BottomRight) => {
                right = (old.x as f32 + old.width as f32 + dx)
                    .clamp(left + 1.0, width as f32);
            }
            _ => {}
        }

        match self.drag_mode {
            Some(Dragmd::Top | Dragmd::TopLeft | Dragmd::TopRight) => {
                top = (old.y as f32 + dy).clamp(0.0, bottom - 1.0);
            }
            _ => {}
        }

        match self.drag_mode {
            Some(Dragmd::Bottom | Dragmd::BottomLeft | Dragmd::BottomRight) => {
                bottom = (old.y as f32 + old.height as f32 + dy)
                    .clamp(top + 1.0, height as f32);
            }
            _ => {}
        }

        left = left.clamp(0.0, width as f32 - 1.0);
        top = top.clamp(0.0, height as f32 - 1.0);
        right = right.clamp(left + 1.0, width as f32);
        bottom = bottom.clamp(top + 1.0, height as f32);

        let x = left.floor() as u32;
        let y = top.floor() as u32;
        let r = right.ceil().min(width as f32) as u32;
        let b = bottom.ceil().min(height as f32) as u32;

        self.crop = Crp {
            x,
            y,
            width: r.saturating_sub(x),
            height: b.saturating_sub(y),
        };
    }
}