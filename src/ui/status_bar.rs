use egui::{Ui, RichText, Color32, ProgressBar};

pub fn status_bar_ui(ui: &mut Ui) {
    ui.horizontal_wrapped(|ui| {
        // === Connection Status ===
        ui.label(RichText::new("Hardware:").strong());
        ui.colored_label(Color32::LIGHT_GREEN, "Connected");

        ui.separator();

        // === CPU Usage ===
        usage_item(ui, "CPU", 0.42);

        ui.separator();

        // === RAM Usage ===
        usage_item(ui, "RAM", 0.67);

        ui.separator();

        // === Disk Usage ===
        usage_item(ui, "Disk", 0.31);
    });
}

/// Helper untuk bikin item usage
fn usage_item(ui: &mut Ui, label: &str, value: f32) {
    ui.label(RichText::new(format!("{}:", label)).strong());
    ui.add(ProgressBar::new(value).desired_width(80.0));
    ui.label(format!("{:.0}%", value * 100.0));
}
