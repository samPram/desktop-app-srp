use eframe::egui::{self, Color32, RichText, Stroke};
use crate::data::models::DynoData;

pub struct DataPanel;

impl DataPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        ui.vertical(|ui| {
            ui.set_min_width(200.0);
            ui.set_max_width(200.0);

            // Live Data & Test Info section header
            ui.heading(RichText::new("Live Data & Test Info").size(16.0));
            ui.add_space(10.0);

            // Sensors section
            ui.label(RichText::new("SENSORS").size(12.0).strong().color(Color32::LIGHT_GRAY));
            ui.add_space(5.0);

            // Air Fuel Ratio
            self.sensor_reading(ui, "Air Fuel Ratio", &format!("{:.1}", data.air_fuel_ratio), "AFR");
            
            // Lambda
            self.sensor_reading(ui, "Lambda", &format!("{:.2}", data.lambda), "L");

            // Intake Temperature
            self.sensor_reading(ui, "Intake Temp (°C)", &format!("{:.1}", data.intake_temp), "°C");

            // Coolant Temperature  
            self.sensor_reading(ui, "Coolant Temp (°C)", &format!("{:.1}", data.coolant_temp), "°C");

            ui.add_space(15.0);

            // Oil Pressure section
            ui.label(RichText::new("Oil Pressure (psi)").size(12.0).strong().color(Color32::LIGHT_GRAY));
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label(RichText::new(&format!("{:.2}", data.oil_pressure)).size(18.0).strong());
            });

            ui.add_space(20.0);

            // Peak Values Cards
            self.peak_value_card(ui, "Peak Horsepower", &format!("{:.1}", data.peak_hp), "HP", Color32::from_rgb(220, 60, 60));
            ui.add_space(10.0);
            self.peak_value_card(ui, "Peak Torque", &format!("{:.1}", data.peak_torque), "Nm", Color32::from_rgb(60, 120, 220));

            ui.add_space(20.0);

            // Test Information section
            ui.label(RichText::new("Test Information").size(14.0).strong());
            ui.add_space(8.0);

            self.test_info_item(ui, "Run ID:", &data.run_id);
            self.test_info_item(ui, "Date:", "2024.07.22");
            self.test_info_item(ui, "Time:", "Yamaha R1");
            self.test_info_item(ui, "Motor:", "YZF-R1 00:32:10");
        });
    }

    fn sensor_reading(&self, ui: &mut egui::Ui, label: &str, value: &str, unit: &str) {
        ui.horizontal(|ui| {
            ui.set_min_height(25.0);
            ui.label(RichText::new(label).size(11.0).color(Color32::LIGHT_GRAY));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new(unit).size(10.0).color(Color32::GRAY));
                ui.label(RichText::new(value).size(13.0).strong());
            });
        });
    }

    fn peak_value_card(&self, ui: &mut egui::Ui, title: &str, value: &str, unit: &str, color: Color32) {
        egui::Frame::none()
            .fill(color.gamma_multiply(0.2))
            .stroke(Stroke::new(1.0, color))
            .rounding(5.0)
            .inner_margin(egui::Margin::symmetric(12.0, 8.0))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(title).size(11.0).color(Color32::LIGHT_GRAY));
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(value).size(20.0).strong().color(Color32::WHITE));
                        ui.label(RichText::new(unit).size(12.0).color(Color32::LIGHT_GRAY));
                    });
                });
            });
    }

    fn test_info_item(&self, ui: &mut egui::Ui, label: &str, value: &str) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(label).size(11.0).color(Color32::LIGHT_GRAY));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new(value).size(11.0));
            });
        });
        ui.add_space(3.0);
    }
}