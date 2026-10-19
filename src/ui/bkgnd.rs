use eframe::egui;
use crate::crp::Crp;

pub fn show(
    ui: &mut egui::Ui,
    texture: Option<&egui::TextureHandle>,
    crop_mode: bool,
    crop: Crp,
    image_width: u32,
    image_height: u32,
) -> Option<egui::Rect> {
    let canvas = ui.available_rect_before_wrap();

    ui.painter().rect_filled(
        canvas,
        0.0,
        egui::Color32::from_gray(28),
    );

    let texture = texture?;

    let size = texture.size_vec2();
    let scale = (canvas.width() / size.x)
        .min(canvas.height() / size.y)
        .min(1.0);

    let image_rect = egui::Rect::from_center_size(
        canvas.center(),
        size * scale,
    );

    ui.painter().image(
        texture.id(),
        image_rect,
        egui::Rect::from_min_max(
            egui::Pos2::ZERO,
            egui::pos2(1.0, 1.0),
        ),
        egui::Color32::WHITE,
    );

    if !crop_mode || image_width == 0 || image_height == 0 {
        return Some(image_rect);
    }

    let to_screen = |x: f32, y: f32| {
        egui::pos2(
            image_rect.left() + x / image_width as f32 * image_rect.width(),
            image_rect.top() + y / image_height as f32 * image_rect.height(),
        )
    };

    let crop_rect = egui::Rect::from_min_max(
        to_screen(crop.x as f32, crop.y as f32),
        to_screen(
            (crop.x + crop.width) as f32,
            (crop.y + crop.height) as f32,
        ),
    );

    let painter = ui.painter();
    let shade = egui::Color32::from_black_alpha(155);
    let areas = [
        egui::Rect::from_min_max(
            image_rect.min,
            egui::pos2(image_rect.right(), crop_rect.top()),
        ),
        egui::Rect::from_min_max(
            egui::pos2(image_rect.left(), crop_rect.bottom()),
            image_rect.max,
        ),
        egui::Rect::from_min_max(
            egui::pos2(image_rect.left(), crop_rect.top()),
            egui::pos2(crop_rect.left(), crop_rect.bottom()),
        ),
        egui::Rect::from_min_max(
            egui::pos2(crop_rect.right(), crop_rect.top()),
            egui::pos2(image_rect.right(), crop_rect.bottom()),
        ),
    ];

    for area in areas {
        if area.is_positive() {
            painter.rect_filled(area, 0.0, shade);
        }
    }

    for i in 1..=2 {
        let t = i as f32 / 3.0;

        let x = egui::lerp(crop_rect.left()..=crop_rect.right(), t);
        let y = egui::lerp(crop_rect.top()..=crop_rect.bottom(), t);
        painter.line_segment(
            [
                egui::pos2(x, crop_rect.top()),
                egui::pos2(x, crop_rect.bottom()),
            ],
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );

        painter.line_segment(
            [
                egui::pos2(crop_rect.left(), y),
                egui::pos2(crop_rect.right(), y),
            ],
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );
    }

    painter.rect_stroke(
        crop_rect,
        0.0,
        egui::Stroke::new(2.0, egui::Color32::WHITE),
        egui::StrokeKind::Inside,
    );

    let handles = [
        crop_rect.left_top(),
        crop_rect.right_top(),
        crop_rect.left_bottom(),
        crop_rect.right_bottom(),
        egui::pos2(crop_rect.center().x, crop_rect.top()),
        egui::pos2(crop_rect.center().x, crop_rect.bottom()),
        egui::pos2(crop_rect.left(), crop_rect.center().y),
        egui::pos2(crop_rect.right(), crop_rect.center().y),
    ];

    for point in handles {
        painter.rect_filled(
            egui::Rect::from_center_size(
                point,
                egui::vec2(8.0, 8.0),
            ),
            1.0,
            egui::Color32::WHITE,
        );
    }

    Some(image_rect)
}
