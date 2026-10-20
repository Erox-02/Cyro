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
    pub(super) img: Option<DynamicImage>,
    pub(super) texture: Option<egui::TextureHandle>,
    pub(super) status: String,
    pub(super) crp: Crp,
    pub(super) crp_mde: bool,
    pub(super) esr_scle: u8,
    pub(super) esr_mdl: crate::esrgan::Model,
    pub(super) drg_mde: Option<Dragmd>,
    pub(super) drg_anc: egui::Pos2,
    pub(super) drg_init: Crp,
    pub(super) esr_run: bool,
    pub(super) esr_rx: Option<
        std::sync::mpsc::Receiver<Result<std::path::PathBuf, String>>
>,
}

impl Default for Cyro {
    fn default() -> Self {
        Self {
            img: None,
            texture: None,
            status: "Open an image to begin.".into(),
            crp: Crp::default(),
            crp_mde: false,
            drg_mde: None,
            drg_anc: egui::Pos2::ZERO,
            drg_init: Crp::default(),
            esr_scle: 4,
            esr_mdl: crate::esrgan::Model::General,
            esr_run: false,
            esr_rx: None,
        }
    }
}
