use eframe::egui;
use super::lgca::{Cyro, Dragmd};
use crate::ui::{bar, bkgnd};
use super::logd;

impl Cyro {
    pub(super) fn show_ui(&mut self, ctx: &egui::Context) {
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
                    let crop_rect = self.crop_screen_rect(image_rect, width, height);

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
                    ui.label(format!("{} × {} px", self.crop.width, self.crop.height));
                }
            });
        });
    }

    fn handle_action(&mut self, action: bar::BarAction, ctx: &egui::Context) {
        match action {
            bar::BarAction::None => {}
            bar::BarAction::Open => logd::open_image(self, ctx),
            bar::BarAction::EnterCrop => logd::start_crop(self),
            bar::BarAction::ApplyCrop => logd::apply_crop(self, ctx),
            bar::BarAction::CancelCrop => logd::cancel_crop(self),
            bar::BarAction::Export => logd::export_image(self),
        }
    }
}