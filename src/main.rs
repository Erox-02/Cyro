mod app;
mod bak;
mod img_wrk;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "CYRO",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app::Cyro::default()))),
    )
}