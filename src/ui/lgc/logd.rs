use eframe::egui;

use super::lgca::Cyro;

pub(super) fn refresh_texture(app: &mut Cyro, ctx: &egui::Context) {
    let Some(image) = &app.img else {
        app.texture = None;
        return;
    };

    let rgba = image.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let color_image =
        egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());

    app.texture = Some(ctx.load_texture(
        "cyro-image",
        color_image,
        egui::TextureOptions::LINEAR,
    ));
}

pub(super) fn open_image(app: &mut Cyro, ctx: &egui::Context) {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("Images", &["png", "jpg", "jpeg"])
        .pick_file()
    else {
        return;
    };

    match image::open(&path) {
        Ok(image) => {
            app.img = Some(image);
            refresh_texture(app, ctx);
            app.crp = crate::crp::Crp::default();
            app.crp_mde = false;
            app.drg_mde = None;
            app.status = format!("opened => {}", path.display());
        }
        Err(err) => {
            app.status = format!("couldnt open image => {err}");
        }
    }
}

pub(super) fn export_image(app: &mut Cyro) {
    let Some(image) = &app.img else {
        app.status = "no image to export.".into();
        return;
    };

    let Some(path) = rfd::FileDialog::new()
        .add_filter("png image", &["png"])
        .set_file_name("exprt.png")
        .save_file()
    else {
        return;
    };

    if path.exists() {
        app.status = "export cancelled => file exists.".into();
        return;
    }

    match image.save_with_format(&path, image::ImageFormat::Png) {
        Ok(()) => app.status = format!("Exported => {}", path.display()),
        Err(err) => app.status = format!("export failed => {err}"),
    }
}

pub(super) fn start_crop(app: &mut Cyro) {
    let Some(image) = &app.img else {
        app.status = "open an image first.".into();
        return;
    };

    let width = image.width();
    let height = image.height();
    let margin_x = (width / 20).min(width.saturating_sub(1));
    let margin_y = (height / 20).min(height.saturating_sub(1));

    app.crp = crate::crp::Crp {
        x: margin_x,
        y: margin_y,
        width: width.saturating_sub(2 * margin_x).max(1),
        height: height.saturating_sub(2 * margin_y).max(1),
    };

    app.crp_mde = true;
    app.drg_mde = None;
    app.status = "drag the white lines to crop.".into();
}

pub(super) fn apply_crop(app: &mut Cyro, ctx: &egui::Context) {
    let result = match &app.img {
        Some(image) => app.crp.crp_img(image),
        None => {
            app.status = "no image loaded.".into();
            return;
        }
    };

    match result {
        Ok(cropped) => {
            let (width, height) = (cropped.width(), cropped.height());
            app.img = Some(cropped);
            app.crp_mde = false;
            app.drg_mde = None;
            app.crp = crate::crp::Crp::default();
            refresh_texture(app, ctx);
            app.status = format!("Cropped => {width} x {height} pixels");
        }
        Err(err) => app.status = format!("crop failed => {err}"),
    }
}

pub(super) fn cancel_crop(app: &mut Cyro) {
    app.crp_mde = false;
    app.drg_mde = None;
    app.crp = crate::crp::Crp::default();
    app.status = "crop cancelled.".into();
}

pub(super) fn esrgan_current(app: &mut Cyro, ctx: &egui::Context) {
    let Some(source) = app.image.as_ref() else {
        return;
    };

    let Some(output) = rfd::FileDialog::new()
        .add_filter("png image", &["png"])
        .set_file_name("upscaled.png")
        .save_file()
    else {
        return;
    };

    let input = std::env::temp_dir().join("cyroin.png");

    if let Err(err) = source.save_with_format(&input, image::ImageFormat::Png) {
        app.status = format!("could not prepare image ++=> {err}");
        return;
    }

    app.status = "upscaling.oO0".into();

    let result = crate::esrgan::upscale(
        &input,
        &output,
        app.esr_scle,
        app.esr_mdl,
    );

    let _ = std::fs::remove_file(&input);

    match result {
        Ok(path) => match image::open(&path) {
            Ok(upscaled) => {
                let (width, height) = (upscaled.width(), upscaled.height());

                app.img = Some(upscaled);
                app.crp_mde = false;
                app.drg_mde = None;
                app.crp = crate::crp::Crp::default();

                refresh_texture(app, ctx);

                app.status = format!("upscaled to {width} x {height} px");
            }
            Err(err) => app.status = format!("cldnt not open output: {err}"),
        },
        Err(err) => app.status = format!("esrgan failed ++=> {err}"),
    }
}