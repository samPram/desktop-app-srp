use crate::data::models::DynoData;
use eframe::egui::{self, Color32, RichText, Stroke};
use eframe::epaint::Margin;
use egui::Frame;

pub struct DataPanel;

impl DataPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        let sidebar_frame = Frame::none().inner_margin(Margin::same(0.0));

        sidebar_frame.show(ui, |ui| {
            ui.vertical(|ui| {
                ui.set_min_width(250.0);

                // Main title
                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new("Live Data & Test Info")
                            .size(18.0)
                            .color(Color32::WHITE),
                    );
                    ui.add_space(20.0);
                });

                // SENSORS section
                self.section_header(ui, "SENSORS");
                ui.add_space(8.0);

                // Create a 3-column grid for sensor readings
                egui::Grid::new("sensors_grid")
                    .num_columns(1)
                    .spacing([8.0, 8.0])
                    .show(ui, |ui| {
                        self.sensor_card(
                            ui,
                            "Air-Fuel Ratio",
                            "",
                            &format!("{}FR", data.air_fuel_ratio as i32),
                        );
                        ui.end_row();
                        self.sensor_card(ui, "Lambda", "", &format!("{:.2}", data.lambda));
                        ui.end_row();
                        self.sensor_card(
                            ui,
                            "Engine Temp (°C)",
                            "",
                            &format!("{:.1}", data.coolant_temp),
                        );
                        ui.end_row();
                        // self.sensor_card(ui, "Oil Pressure (°C)", "", &format!("{:.1}", data.oil_pressure_temp));
                        // ui.end_row();
                        self.sensor_card(
                            ui,
                            "Intake Temp °C",
                            "",
                            &format!("{:.1}", data.intake_temp),
                        );
                        ui.end_row();
                    });

                ui.add_space(20.0);

                // Oil Pressure section
                ui.label(
                    RichText::new("Oil Pressure (psi)")
                        .size(14.0)
                        .strong()
                        .color(Color32::WHITE),
                );
                ui.add_space(8.0);

                // Large oil pressure reading
                egui::Frame::none()
                    .fill(Color32::from_rgb(40, 40, 40))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 60)))
                    .rounding(8.0)
                    .inner_margin(egui::Margin::symmetric(15.0, 12.0))
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(&format!("{:.2}", data.oil_pressure))
                                    .size(24.0)
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });
                    });

                ui.add_space(25.0);

                // Peak Values section
                ui.vertical(|ui| {
                    // Peak Horsepower card
                    self.large_metric_card(
                        ui,
                        "Max Horsepower",
                        &format!("{:.1}", data.peak_hp),
                        "HP",
                        Color32::from_rgb(180, 50, 50),
                    );

                    ui.add_space(12.0);

                    // Peak Torque card
                    self.large_metric_card(
                        ui,
                        "Max Torque",
                        &format!("{:.1}", data.peak_torque),
                        "Nm",
                        Color32::from_rgb(50, 100, 180),
                    );
                });

                ui.add_space(20.0);

                // Test Information section
                ui.label(
                    RichText::new("Test Information")
                        .size(16.0)
                        .strong()
                        .color(Color32::WHITE),
                );
                ui.add_space(10.0);

                // Test info items
                self.test_info_item(ui, "Run ID:", &format!("#{:03}", data.run_id));
                self.test_info_item(ui, "Date:", "2024-07-26");
                self.test_info_item(ui, "Motorcycle:", "Yamaha R1");
                self.test_info_item(ui, "Time:", "00:02:10");

                ui.add_space(20.0);
            });
        });
    }

    fn section_header(&self, ui: &mut egui::Ui, title: &str) {
        ui.label(
            RichText::new(title)
                .size(12.0)
                .strong()
                .color(Color32::LIGHT_GRAY),
        );
    }

    fn sensor_card(&self, ui: &mut egui::Ui, label: &str, unit_suffix: &str, value: &str) {
        let width = 70.0;
        let height = 50.0;

        egui::Frame::none()
            .fill(Color32::from_rgb(35, 35, 35))
            .stroke(Stroke::new(1.0, Color32::from_rgb(55, 55, 55)))
            .rounding(6.0)
            .inner_margin(egui::Margin::symmetric(8.0, 6.0))
            .show(ui, |ui| {
                ui.set_min_size([width, height].into());
                ui.vertical_centered(|ui| {
                    // Label
                    ui.label(RichText::new(label).size(14.0).color(Color32::LIGHT_GRAY));
                    ui.add_space(2.0);

                    // Unit suffix (like AFR)
                    if !unit_suffix.is_empty() {
                        ui.label(RichText::new(unit_suffix).size(8.0).color(Color32::GRAY));
                    }

                    ui.add_space(2.0);

                    // Value
                    ui.label(
                        RichText::new(value)
                            .size(24.0)
                            .strong()
                            .color(Color32::WHITE),
                    );
                });
            });
    }

    fn large_metric_card(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        value: &str,
        unit: &str,
        color: Color32,
    ) {
        let available_width = ui.available_width();

        egui::Frame::none()
            .fill(color.gamma_multiply(0.3))
            .stroke(Stroke::new(2.0, color))
            .rounding(8.0)
            .inner_margin(egui::Margin::symmetric(16.0, 12.0))
            .show(ui, |ui| {
                ui.set_min_width(available_width - 32.0);
                ui.vertical_centered(|ui| {
                    // Title
                    ui.label(RichText::new(title).size(12.0).color(Color32::LIGHT_GRAY));
                    ui.add_space(8.0);

                    // Value and unit
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        ui.label(
                            RichText::new(value)
                                .size(28.0)
                                .strong()
                                .color(Color32::WHITE),
                        );
                        ui.label(RichText::new(unit).size(16.0).color(Color32::LIGHT_GRAY));
                    });
                });
            });
    }

    fn test_info_item(&self, ui: &mut egui::Ui, label: &str, value: &str) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(label).size(11.0).color(Color32::LIGHT_GRAY));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new(value).size(11.0).color(Color32::WHITE));
            });
        });
        ui.add_space(4.0);
    }
}
