use eframe::egui;

use super::lgca::Cyro;

pub(super) fn refresh_texture(app: &mut Cyro, ctx: &egui::Context) {
    let Some(image) = &app.image else {
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
            app.image = Some(image);
            refresh_texture(app, ctx);
            app.crop = crate::crp::Crp::default();
            app.crop_mode = false;
            app.drag_mode = None;
            app.status = format!("opened => {}", path.display());
        }
        Err(err) => {
            app.status = format!("Could not open image => {err}");
        }
    }
}

pub(super) fn export_image(app: &mut Cyro) {
    let Some(image) = &app.image else {
        app.status = "No image to export.".into();
        return;
    };

    let Some(path) = rfd::FileDialog::new()
        .add_filter("PNG image", &["png"])
        .set_file_name("export.png")
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
        Err(err) => app.status = format!("Export failed => {err}"),
    }
}

pub(super) fn start_crop(app: &mut Cyro) {
    let Some(image) = &app.image else {
        app.status = "open an image first.".into();
        return;
    };

    let width = image.width();
    let height = image.height();
    let margin_x = (width / 20).min(width.saturating_sub(1));
    let margin_y = (height / 20).min(height.saturating_sub(1));

    app.crop = crate::crp::Crp {
        x: margin_x,
        y: margin_y,
        width: width.saturating_sub(2 * margin_x).max(1),
        height: height.saturating_sub(2 * margin_y).max(1),
    };

    app.crop_mode = true;
    app.drag_mode = None;
    app.status = "drag the white handles to crop.".into();
}

pub(super) fn apply_crop(app: &mut Cyro, ctx: &egui::Context) {
    let result = match &app.image {
        Some(image) => app.crop.crp_img(image),
        None => {
            app.status = "no image loaded.".into();
            return;
        }
    };

    match result {
        Ok(cropped) => {
            let (width, height) = (cropped.width(), cropped.height());
            app.image = Some(cropped);
            app.crop_mode = false;
            app.drag_mode = None;
            app.crop = crate::crp::Crp::default();
            refresh_texture(app, ctx);
            app.status = format!("Cropped => {width} × {height} pixels");
        }
        Err(err) => app.status = format!("Crop failed => {err}"),
    }
}

pub(super) fn cancel_crop(app: &mut Cyro) {
    app.crop_mode = false;
    app.drag_mode = None;
    app.crop = crate::crp::Crp::default();
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
        app.esrgan_scale,
        app.esrgan_model,
    );

    let _ = std::fs::remove_file(&input);

    match result {
        Ok(path) => match image::open(&path) {
            Ok(upscaled) => {
                let (width, height) = (upscaled.width(), upscaled.height());

                app.image = Some(upscaled);
                app.crop_mode = false;
                app.drag_mode = None;
                app.crop = crate::crp::Crp::default();

                refresh_texture(app, ctx);

                app.status = format!("upscaled to {width} x {height} px");
            }
            Err(err) => app.status = format!("cldnt not open output: {err}"),
        },
        Err(err) => app.status = format!("esrgan failed ++=> {err}"),
    }
}