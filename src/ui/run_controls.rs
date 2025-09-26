use crate::data::models::DynoData;
use eframe::egui::{self, Button, Color32, RichText};

#[derive(Clone, Default)]
pub struct RunControls {
    is_running: bool,
    emergency_stop: bool,
}

impl RunControls {
    pub fn new() -> Self {
        Self {
            is_running: false,
            emergency_stop: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, data: &mut DynoData) {
        ui.horizontal(|ui| {
            // Start/Stop button
            let start_stop_text = if self.is_running {
                "Stop Test"
            } else {
                "Start Test"
            };
            let start_stop_color = if self.is_running {
                Color32::from_rgb(220, 60, 60)
            } else {
                Color32::from_rgb(60, 180, 60)
            };

            if ui
                .add(
                    Button::new(
                        RichText::new(start_stop_text)
                            .size(14.0)
                            .color(Color32::WHITE),
                    )
                    .fill(start_stop_color)
                    .min_size([100.0, 35.0].into()),
                )
                .clicked()
            {
                self.is_running = !self.is_running;
                if !self.is_running {
                    // Reset emergency stop when stopping test
                    self.emergency_stop = false;
                }
            }

            ui.add_space(10.0);

            // Emergency stop button
            if ui
                .add(
                    Button::new(
                        RichText::new("EMERGENCY STOP")
                            .size(14.0)
                            .color(Color32::WHITE),
                    )
                    .fill(Color32::from_rgb(180, 0, 0))
                    .min_size([140.0, 35.0].into()),
                )
                .clicked()
            {
                self.emergency_stop = true;
                self.is_running = false;
            }

            ui.add_space(10.0);

            // Reset button
            if ui
                .add(
                    Button::new(RichText::new("Reset").size(14.0).color(Color32::WHITE))
                        .fill(Color32::from_rgb(80, 80, 80))
                        .min_size([80.0, 35.0].into()),
                )
                .clicked()
            {
                self.reset();
                // Reset data as well
                *data = DynoData::new();
            }

            ui.add_space(20.0);

            // Status indicator
            let status_text = if self.emergency_stop {
                "EMERGENCY STOP ACTIVE"
            } else if self.is_running {
                "Test Running"
            } else {
                "Ready"
            };

            let status_color = if self.emergency_stop {
                Color32::from_rgb(220, 60, 60)
            } else if self.is_running {
                Color32::from_rgb(60, 180, 60)
            } else {
                Color32::from_rgb(220, 180, 60)
            };

            ui.label(RichText::new(status_text).size(14.0).color(status_color));
        });
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn is_emergency_stop(&self) -> bool {
        self.emergency_stop
    }

    pub fn reset(&mut self) {
        self.is_running = false;
        self.emergency_stop = false;
    }
}
