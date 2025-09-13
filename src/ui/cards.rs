use egui::{Ui, RichText, Frame, Color32, Stroke};

pub struct PeakCard {
    pub title: &'static str,
    pub value: String,
    pub color: Color32,
}

impl PeakCard {
    pub fn ui(&self, ui: &mut Ui) {
        Frame::group(ui.style())
            .stroke(Stroke::new(1.0, self.color))
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(egui::Margin::symmetric(12, 8))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(self.title).size(14.0).strong());
                    ui.label(RichText::new(&self.value).size(20.0).color(self.color));
                });
            });
    }
}

pub fn peak_cards_ui(ui: &mut Ui) {
    let horsepower = PeakCard {
        title: "Peak Horsepower",
        value: "125.7 HP".to_string(),
        color: Color32::RED,
    };

    let torque = PeakCard {
        title: "Peak Torque",
        value: "98.3 Nm".to_string(),
        color: Color32::BLUE,
    };

    ui.horizontal(|ui| {
        horsepower.ui(ui);
        ui.add_space(12.0);
        torque.ui(ui);
    });
}
