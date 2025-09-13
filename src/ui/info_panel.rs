use egui::{Ui, RichText, Color32, Frame, Stroke};

pub fn info_panel_ui(ui: &mut Ui) {
    ui.vertical(|ui| {
        // === Sensors ===
        section_card(ui, "SENSORS", |ui| {
            ui.label("Air-Fuel Ratio: 13FR");
            ui.label("AFR: 13.20");
            ui.label("Lambda: 0.90");
            ui.label("Engine Temp (°C): 98.5");
            ui.label("Oil Temp (°C): 65.0");
            ui.label("Intake Temp (°C): 25.3");
        });

        ui.add_space(12.0);

        // === Oil Pressure ===
        section_card(ui, "Oil Pressure (psi)", |ui| {
            ui.label(RichText::new("6590.00").size(18.0).color(Color32::LIGHT_BLUE));
        });

        ui.add_space(12.0);

        // === Test Information ===
        section_card(ui, "Test Information", |ui| {
            ui.label("Run ID: #008");
            ui.label("Date: 2024-07-26");
            ui.label("Bike: Yamaha R1");
            ui.label("Duration: 00:02:10");
        });
    });
}

/// Helper untuk bikin card section
fn section_card<R>(ui: &mut Ui, title: &str, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::group(ui.style())
        .stroke(Stroke::new(1.0, Color32::GRAY))
        .corner_radius(egui::CornerRadius::same(6))
        .inner_margin(egui::Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.heading(RichText::new(title).size(14.0).strong());
            ui.separator();
            add_contents(ui)
        })
        .inner
}
