use eframe::egui;

use crate::crp::Crp;

use super::lgca::{Cyro, Dragmd};

impl Cyro {
    pub(super) fn crop_screen_rect(
        &self,
        image_rect: egui::Rect,
        width: u32,
        height: u32,
    ) -> egui::Rect {
        egui::Rect::from_min_max(
            Self::pixel_to_screen(
                egui::pos2(self.crp.x as f32, self.crp.y as f32),
                image_rect,
                width,
                height,
            ),
            Self::pixel_to_screen(
                egui::pos2(
                    (self.crp.x + self.crp.width) as f32,
                    (self.crp.y + self.crp.height) as f32,
                ),
                image_rect,
                width,
                height,
            ),
        )
    }

    pub(super) fn pixel_to_screen(
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

    pub(super) fn screen_to_pixel(
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

    pub(super) fn hit_test(pos: egui::Pos2, rect: egui::Rect) -> Dragmd {
        let near = |p: egui::Pos2| (pos - p).length() < 14.0;

        if near(rect.left_top()) {
            Dragmd::TopLeft
        } else if near(rect.right_top()) {
            Dragmd::TopRight
        } else if near(rect.left_bottom()) {
            Dragmd::BottomLeft
        } else if near(rect.right_bottom()) {
            Dragmd::BottomRight
        } else if near(egui::pos2(rect.center().x, rect.top())) {
            Dragmd::Top
        } else if near(egui::pos2(rect.center().x, rect.bottom())) {
            Dragmd::Bottom
        } else if near(egui::pos2(rect.left(), rect.center().y)) {
            Dragmd::Left
        } else if near(egui::pos2(rect.right(), rect.center().y)) {
            Dragmd::Right
        } else {
            Dragmd::Move
        }
    }

    pub(super) fn update_drag(
        &mut self,
        pointer: egui::Pos2,
        rect: egui::Rect,
        width: u32,
        height: u32,
    ) {
        if width == 0 || height == 0 || rect.width() <= 0.0 || rect.height() <= 0.0 {
            return;
        }

        let point = Self::screen_to_pixel(pointer, rect, width, height);
        let dx = point.x - self.drg_anc.x;
        let dy = point.y - self.drg_anc.y;
        let old = self.drg_init;

        let mut left = old.x as f32;
        let mut top = old.y as f32;
        let mut right = (old.x + old.width) as f32;
        let mut bottom = (old.y + old.height) as f32;

        match self.drg_mde {
            Some(Dragmd::New) => {
                left = self.drg_anc.x.min(point.x);
                top = self.drg_anc.y.min(point.y);
                right = self.drg_anc.x.max(point.x);
                bottom = self.drg_anc.y.max(point.y);
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

        match self.drg_mde {
            Some(Dragmd::Right | Dragmd::TopRight | Dragmd::BottomRight) => {
                right = (old.x as f32 + old.width as f32 + dx)
                    .clamp(left + 1.0, width as f32);
            }
            _ => {}
        }

        match self.drg_mde {
            Some(Dragmd::Top | Dragmd::TopLeft | Dragmd::TopRight) => {
                top = (old.y as f32 + dy).clamp(0.0, bottom - 1.0);
            }
            _ => {}
        }

        match self.drg_mde {
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

        self.crp = Crp {
            x,
            y,
            width: r.saturating_sub(x),
            height: b.saturating_sub(y),
        };
    }
}