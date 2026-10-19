pub mod bkgnd;
pub mod bar;
pub mod lgc;

pub use lgc::Cyro;

use eframe::egui;

impl eframe::App for Cyro {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        self.show_ui(ctx);
    }
}
