use egui::{Ui, menu, Label, RichText};

pub fn topbar_ui(ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            // === Left Menu Items ===
            ui.menu_button("File", |ui| {
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });

            ui.menu_button("Settings", |ui| {
                ui.label("Preferences (coming soon)");
            });

            ui.menu_button("Runs", |ui| {
                ui.label("Load / Save Runs");
            });

            ui.menu_button("Help", |ui| {
                ui.label("About MotoDyno");
            });

            // === Center Title ===
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                ui.add(Label::new(
                    RichText::new("MotoDyno v1.0.0").strong().size(16.0)
                ));
            });

            // === Right Side Placeholder ===
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("⚙️");
            });
        });
}
