use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarAction {
    None,
    Open,
    EnterCrop,
    ApplyCrop,
    CancelCrop,
    Export,
}

pub fn show(ui: &mut egui::Ui, crop_mode: bool) -> BarAction {
    let mut action = BarAction::None;

    ui.horizontal(|ui| {
        if ui.button("Open").clicked() {
            action = BarAction::Open;
        }
        ui.separator();

        if crop_mode {
            if ui.button("Apply crop").clicked() {
                action = BarAction::ApplyCrop;
            }

            if ui.button("Cancel").clicked() {
                action = BarAction::CancelCrop;
            }
        } else if ui.button("Crop").clicked() {
            action = BarAction::EnterCrop;
        }

        ui.separator();

        if ui.button("Export PNG").clicked() {
            action = BarAction::Export;
        }
    });

    action
}
