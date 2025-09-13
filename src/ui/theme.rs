use egui::{Context, Visuals, Style, FontId, TextStyle, FontFamily, Stroke, Color32};

pub fn apply_dark_theme(ctx: &Context) {
    let mut style: Style = (*ctx.style()).clone();

    // === Base Visuals ===
    style.visuals = Visuals::dark();

    // Background warna utama (dark gray agak kebiruan)
    style.visuals.panel_fill = Color32::from_rgb(24, 25, 28);
    style.visuals.window_fill = Color32::from_rgb(28, 29, 34);

    // Border & stroke
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_gray(60));
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_gray(100));

    // === Font & Text ===
    style.text_styles = [
        (TextStyle::Heading, FontId::new(20.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Small, FontId::new(12.0, FontFamily::Proportional)),
    ].into();

    // Apply ke context
    ctx.set_style(style);
}
