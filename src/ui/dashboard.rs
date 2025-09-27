use crate::data::models::DynoData;
use crate::ui::charts::PowerTorqueChart;
use crate::ui::data_panel::DataPanel;
use crate::ui::gauges::CircularGauge;
use crate::ui::panels::PanelLayout;
use crate::ui::rpm_gauge::RpmGauge;
use crate::ui::sidebar::{Sidebar, SidebarItem};
use crate::ui::speed_gauge::SpeedGauge;
use eframe::egui::{self, Color32, RichText, Stroke};

pub struct Dashboard {
    sidebar: Sidebar,
    rpm_gauge: CircularGauge,
    speed_gauge: CircularGauge,
    chart: PowerTorqueChart,
    data_panel: DataPanel,
    panel_layout: PanelLayout,
    use_panel_layout: bool,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            sidebar: Sidebar::new(),
            rpm_gauge: CircularGauge::new(
                "RPM",
                0.0,
                12000.0,
                "RPM",
                Color32::from_rgb(220, 60, 60),
            ),
            speed_gauge: CircularGauge::new(
                "Speed (km/h)",
                0.0,
                300.0,
                "km/h",
                Color32::from_rgb(60, 120, 220),
            ),
            chart: PowerTorqueChart::new(),
            data_panel: DataPanel::new(),
            panel_layout: PanelLayout::new(),
            use_panel_layout: true,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, data: &mut DynoData) {
        // Set dark theme
        let mut style = (*ui.ctx().style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.override_text_color = Some(Color32::WHITE);
        style.visuals.panel_fill = Color32::from_rgb(30, 30, 30);
        style.visuals.window_fill = Color32::from_rgb(25, 25, 25);
        ui.ctx().set_style(style);

        // Choose layout mode
        if self.use_panel_layout {
            // Panel layout mode - sidebar is integrated in left panel
            self.panel_layout.show(ui, data);
        } else {
            // Classic layout mode - separate sidebar
            ui.horizontal(|ui| {
                // Sidebar
                egui::Frame::none()
                    .fill(Color32::from_rgb(35, 35, 35))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(50, 50, 50)))
                    .show(ui, |ui| {
                        self.sidebar.show(ui);
                    });

                ui.separator();

                // Main content area
                ui.vertical(|ui| match self.sidebar.selected_item() {
                    SidebarItem::Dashboard => self.show_dashboard_content(ui, data),
                    _ => {
                        ui.centered_and_justified(|ui| {
                            ui.label(RichText::new("Feature coming soon...").size(18.0));
                        });
                    }
                });
            });
        }
    }

    fn show_dashboard_content(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        // Status bar at bottom
        let _available_height = ui.available_height() - 30.0; // Reserve space for status bar

        ui.horizontal(|ui| {
            // Center panel as a card with margin
            let center_frame = egui::Frame::none()
                .fill(Color32::from_rgb(35, 35, 35))
                .stroke(Stroke::new(1.0, Color32::from_rgb(50, 50, 50)))
                .rounding(12.0)
                .inner_margin(egui::Margin::same(20.0));

            center_frame.show(ui, |ui| {
                ui.set_width(ui.available_width() - 240.0); // Reserve space for data panel

                ui.vertical(|ui| {
                    // Gauges row
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);

                        // RPM Gauge
                        ui.vertical(|ui| {
                            self.rpm_gauge.show(ui, data.rpm);
                        });

                        ui.add_space(40.0);

                        // Speed Gauge
                        ui.vertical(|ui| {
                            self.speed_gauge.show(ui, data.speed_kmh);
                        });

                        // Add torque and HP widgets on the right side of gauges
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(20.0);

                            // Torque widget - current value
                            ui.vertical(|ui| {
                                self.torque_hp_widget(
                                    ui,
                                    "Torque",
                                    &format!("{:.1}", data.torque),
                                    "Nm",
                                    Color32::from_rgb(50, 100, 180),
                                );
                            });

                            ui.add_space(20.0);

                            // HP widget - current value
                            ui.vertical(|ui| {
                                self.torque_hp_widget(
                                    ui,
                                    "Power",
                                    &format!("{:.1}", data.horsepower),
                                    "HP",
                                    Color32::from_rgb(180, 50, 50),
                                );
                            });
                        });
                    });

                    ui.add_space(20.0);

                    // Chart section
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.vertical(|ui| {
                            self.chart.show(ui, &data.power_curve, &data.torque_curve);
                        });
                    });
                });
            });

            ui.add_space(10.0);
            ui.separator();

            // Right section - data panel
            ui.vertical(|ui| {
                ui.add_space(10.0);
                self.data_panel.show(ui, data);
            });
        });

        // Status bar at bottom
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                let status_color = if data.is_hardware_connected {
                    Color32::from_rgb(60, 180, 60)
                } else if data.connection_status.contains("Error") {
                    Color32::from_rgb(220, 60, 60)
                } else if data.connection_status.contains("Connecting") {
                    Color32::from_rgb(220, 180, 60)
                } else if data.connection_status.contains("Simulation") {
                    Color32::from_rgb(0, 255, 255)
                } else {
                    Color32::LIGHT_GRAY
                };
                ui.label(
                    RichText::new(&format!("Status: {}", data.connection_status))
                        .size(11.0)
                        .color(status_color),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new("Disk: 94.4% Free")
                            .size(11.0)
                            .color(Color32::LIGHT_GRAY),
                    );
                    ui.label(
                        RichText::new("RAM: 2.5GB")
                            .size(11.0)
                            .color(Color32::LIGHT_GRAY),
                    );
                    ui.label(
                        RichText::new("CPU: 15%")
                            .size(11.0)
                            .color(Color32::LIGHT_GRAY),
                    );
                });
            });
            ui.add_space(5.0);
        });
    }

    // Panel layout control methods
    pub fn toggle_panel_layout(&mut self) {
        self.use_panel_layout = !self.use_panel_layout;
    }

    pub fn is_using_panel_layout(&self) -> bool {
        self.use_panel_layout
    }

    pub fn toggle_top_panel(&mut self) {
        self.panel_layout.toggle_top_panel();
    }

    pub fn toggle_left_panel(&mut self) {
        self.panel_layout.toggle_left_panel();
    }

    pub fn toggle_right_panel(&mut self) {
        self.panel_layout.toggle_right_panel();
    }

    pub fn toggle_bottom_panel(&mut self) {
        self.panel_layout.toggle_bottom_panel();
    }

    pub fn reset_panel_layout(&mut self) {
        self.panel_layout.reset_layout();
    }

    // Panel visibility getters
    pub fn is_top_panel_visible(&self) -> bool {
        self.panel_layout.is_top_panel_visible()
    }

    pub fn is_left_panel_visible(&self) -> bool {
        self.panel_layout.is_left_panel_visible()
    }

    pub fn is_right_panel_visible(&self) -> bool {
        self.panel_layout.is_right_panel_visible()
    }

    pub fn is_bottom_panel_visible(&self) -> bool {
        self.panel_layout.is_bottom_panel_visible()
    }

    fn torque_hp_widget(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        value: &str,
        unit: &str,
        color: Color32,
    ) {
        egui::Frame::none()
            .fill(color.gamma_multiply(0.3))
            .stroke(Stroke::new(2.0, color))
            .rounding(8.0)
            .inner_margin(egui::Margin::symmetric(16.0, 12.0))
            .show(ui, |ui| {
                ui.set_min_size([120.0, 80.0].into());
                ui.vertical_centered(|ui| {
                    // Title
                    ui.label(RichText::new(title).size(12.0).color(Color32::LIGHT_GRAY));
                    ui.add_space(4.0);

                    // Value and unit
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        ui.label(
                            RichText::new(value)
                                .size(20.0)
                                .strong()
                                .color(Color32::WHITE),
                        );
                        ui.label(RichText::new(unit).size(12.0).color(Color32::LIGHT_GRAY));
                    });
                });
            });
    }
}
