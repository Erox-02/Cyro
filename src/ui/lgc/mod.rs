mod lgca;
mod lgcb;
mod lgcg;
mod logd;

pub use lgca::Cyro;

use eframe::egui;

impl eframe::App for Cyro {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_ui(ctx);
    }
}