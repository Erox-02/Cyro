use eframe::egui;
use image::DynamicImage;

use crate::crp::Crp;

#[derive(Clone, Copy)]
pub(super) enum Dragmd {
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
    pub(super) image: Option<DynamicImage>,
    pub(super) texture: Option<egui::TextureHandle>,
    pub(super) status: String,
    pub(super) crop: Crp,
    pub(super) crop_mode: bool,
    pub(super) esrgan_scale: u8,
    pub(super) esrgan_model: crate::esrgan::Model,
    pub(super) drag_mode: Option<Dragmd>,
    pub(super) drag_anchor: egui::Pos2,
    pub(super) drag_initial: Crp,
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
            esrgan_scale: 4,
            esrgan_model: crate::esrgan::Model::General,
        }
    }
}
