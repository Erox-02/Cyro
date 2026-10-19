mod ui;
mod crp;
mod img_wrk;
mod esrgan;
mod bak;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "CYRO",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(ui::Cyro::default()))),
    )
}
