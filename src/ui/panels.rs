use crate::data::models::DynoData;
use crate::ui::charts::PowerTorqueChart;
use crate::ui::gauges::CircularGauge;
use crate::ui::rpm_gauge::RpmGauge;
use crate::ui::run_controls::RunControls;
use crate::ui::sidebar::{Sidebar, SidebarItem};
use crate::ui::speed_gauge::SpeedGauge;
use eframe::egui::{self, Color32, Frame, Margin, RichText, Rounding, Stroke, Vec2};

pub struct PanelLayout {
    chart: PowerTorqueChart,
    // speed_gauge: CircularGauge,
    // rpm_gauge: CircularGauge,
    rpm_gauge: RpmGauge,
    speed_gauge: SpeedGauge,
    run_controls: RunControls,
    sidebar: Sidebar,
    show_top_panel: bool,
    show_left_panel: bool,
    show_right_panel: bool,
    show_bottom_panel: bool,
}

impl PanelLayout {
    pub fn new() -> Self {
        Self {
            rpm_gauge: RpmGauge::new(),
            speed_gauge: SpeedGauge::new(),
            chart: PowerTorqueChart::new(),
            run_controls: RunControls::new(),
            sidebar: Sidebar::new(),
            show_top_panel: true,
            show_left_panel: true,
            show_right_panel: true,
            show_bottom_panel: true,
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

        // Top panel - Controls and status
        if self.show_top_panel {
            egui::TopBottomPanel::top("top_panel")
                .resizable(true)
                .min_height(80.0)
                .default_height(100.0)
                .frame(
                    egui::Frame::none()
                        .fill(Color32::from_rgb(35, 35, 35))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(50, 50, 50))),
                )
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            // ui.heading(RichText::new("Dyno Control Panel").size(20.0).color(Color32::WHITE));
                            ui.horizontal(|ui| {
                                self.run_controls.show(ui, data);
                            });
                        });
                    });
                });
        }

        // Bottom panel - Status bar and system info
        if self.show_bottom_panel {
            egui::TopBottomPanel::bottom("bottom_panel")
                .resizable(true)
                .min_height(40.0)
                .default_height(50.0)
                .frame(
                    egui::Frame::none()
                        .fill(Color32::from_rgb(25, 25, 25))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(50, 50, 50))),
                )
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new("Status: Connected to Dyno Hardware. Ready for test.")
                                .size(11.0)
                                .color(Color32::LIGHT_GRAY),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(10.0);
                            ui.label(
                                RichText::new("Disk: 94.4% Free")
                                    .size(11.0)
                                    .color(Color32::LIGHT_GRAY),
                            );
                            ui.separator();
                            ui.label(
                                RichText::new("RAM: 2.5GB")
                                    .size(11.0)
                                    .color(Color32::LIGHT_GRAY),
                            );
                            ui.separator();
                            ui.label(
                                RichText::new("CPU: 15%")
                                    .size(11.0)
                                    .color(Color32::LIGHT_GRAY),
                            );
                        });
                    });
                });
        }

        // Left panel - Navigation Sidebar
        if self.show_left_panel {
            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(220.0) // Match sidebar min width
                .width_range(150.0..=280.0)
                .frame(
                    egui::Frame::none() // Remove frame since sidebar handles styling
                        .fill(Color32::TRANSPARENT) // Let sidebar handle background
                        .stroke(Stroke::NONE),
                )
                .show_inside(ui, |ui| {
                    if let Some(selected) = self.sidebar.show(ui) {
                        // Handle navigation changes
                        match selected {
                            SidebarItem::Dashboard => { /* switch to dashboard */ }
                            SidebarItem::DynData => { /* switch to dyn data */ }
                            SidebarItem::RunHistory => { /* switch to run history */ }
                            SidebarItem::Reports => { /* switch to reports */ }
                            SidebarItem::Configuration => { /* switch to configuration */ }
                        }
                    }
                });
        }

        // Right panel - Data display with margins and fixed bottom section
        if self.show_right_panel {
            egui::SidePanel::right("right_panel")
                .resizable(true)
                .default_width(280.0)
                .width_range(260.0..=320.0)
                .frame(
                    egui::Frame::none()
                        .fill(Color32::from_rgb(25, 25, 25))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(45, 45, 45)))
                        .inner_margin(Margin::same(15.0)), // Add margins to the panel
                )
                .show_inside(ui, |ui| {
                    // Use vertical layout with fixed bottom section
                    ui.vertical(|ui| {
                        // Top section - Live Data (scrollable)
                        let available_height = ui.available_height();
                        let bottom_section_height = 120.0; // Reserve space for Test Information
                        let live_data_height = available_height - bottom_section_height - 20.0; // 20px spacing

                        ui.allocate_ui_with_layout(
                            Vec2::new(ui.available_width(), live_data_height),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                egui::ScrollArea::vertical()
                                    .auto_shrink([false, true])
                                    .show(ui, |ui| {
                                        self.show_live_data_section(ui, data);
                                    });
                            },
                        );

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        // Bottom section - Test Information (fixed)
                        self.show_test_information_section(ui, data);
                    });
                });
        }

        // Central panel - Main content area based on sidebar selection
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(25, 25, 25))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(50, 50, 50))),
            )
            .show_inside(ui, |ui| match self.sidebar.selected_item() {
                SidebarItem::Dashboard => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.show_dashboard_content(ui, data);
                    });
                }
                _ => {
                    ui.centered_and_justified(|ui| {
                        ui.label(RichText::new("Feature coming soon...").size(18.0));
                    });
                }
            });
    }

    // Main dashboard container with proper responsive sizing
    fn show_dashboard_content(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        let center_frame = Frame::none().inner_margin(Margin::same(0.0));

        center_frame.show(ui, |ui| {
            ui.set_width(ui.available_width() - 300.0);

            ui.vertical(|ui| {

                // ui.vertical_centered(|ui| {
                //     ui.add_space(20.0);
                //
                //     // Top section - Gauges Card (responsive width)
                //     self.show_gauges_card(ui, data);
                //
                //     ui.add_space(20.0);
                //
                //     // Middle section - Chart Card (responsive width)
                //     self.show_chart_card(ui, data);
                //
                //     ui.add_space(20.0);
                //
                //     // Bottom section - Performance Values Cards (responsive layout)
                //     self.show_performance_cards(ui, data);
                //
                //     ui.add_space(20.0);
                // });

                ui.horizontal(|ui| {
                    // RPM Gauge
                    ui.vertical(|ui| {
                        self.rpm_gauge.show(ui, data.rpm); // assuming your data has rpm field
                    });

                    ui.add_space(20.0); // Space between gauges

                    // Speed Gauge
                    ui.vertical(|ui| {
                        self.speed_gauge.show(ui, data.speed_kmh); // assuming your data has speed field
                    });
                });
            });
        });
    }

    fn show_gauges_card(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        // Calculate responsive width with padding
        let available_width = ui.available_width();
        let max_card_width = 800.0;
        let card_width = available_width.min(max_card_width) - 40.0; // Padding on sides

        // Center the card
        ui.horizontal(|ui| {
            let padding = (available_width - card_width).max(0.0) / 2.0;
            ui.add_space(padding);

            // Gauges container card
            let card_frame = Frame::none()
                .fill(Color32::from_rgb(40, 40, 45))
                .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 65)))
                .rounding(Rounding::same(12.0))
                .inner_margin(Margin::same(20.0));

            card_frame.show(ui, |ui| {
                ui.set_width(card_width);

                // Card header
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("Engine Monitoring")
                            .size(18.0)
                            .strong()
                            .color(Color32::WHITE),
                    );
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("●")
                                .size(12.0)
                                .color(Color32::from_rgb(0, 255, 0)),
                        );
                        ui.label(
                            RichText::new("Connected to Dyno Hardware. Ready for test.")
                                .size(12.0)
                                .color(Color32::LIGHT_GRAY),
                        );
                    });
                });

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(15.0);

                // Responsive gauge layout
                if card_width > 500.0 {
                    // Wide layout - horizontal
                    ui.horizontal_centered(|ui| {
                        // RPM Gauge
                        ui.vertical(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new("RPM")
                                        .size(16.0)
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                            ui.add_space(10.0);
                            self.rpm_gauge.show(ui, data.rpm);
                        });

                        ui.add_space(40.0);

                        // Speed Gauge
                        ui.vertical(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new("Speed (km/h)")
                                        .size(16.0)
                                        .strong()
                                        .color(Color32::WHITE),
                                );
                            });
                            ui.add_space(10.0);
                            self.speed_gauge.show(ui, data.speed_kmh);
                        });
                    });
                } else {
                    // Narrow layout - vertical
                    ui.vertical_centered(|ui| {
                        // RPM Gauge
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new("RPM")
                                    .size(16.0)
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                            ui.add_space(10.0);
                            self.rpm_gauge.show(ui, data.rpm);
                        });

                        ui.add_space(20.0);

                        // Speed Gauge
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new("Speed (km/h)")
                                    .size(16.0)
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                            ui.add_space(10.0);
                            self.speed_gauge.show(ui, data.speed_kmh);
                        });
                    });
                }
            });
        });
    }

    fn show_chart_card(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        // Calculate responsive width with padding
        let available_width = ui.available_width();
        let max_card_width = 900.0;
        let card_width = available_width.min(max_card_width) - 40.0; // Padding on sides

        // Center the card
        ui.horizontal(|ui| {
            let padding = (available_width - card_width).max(0.0) / 2.0;
            ui.add_space(padding);

            // Chart container card
            let card_frame = Frame::none()
                .fill(Color32::from_rgb(40, 40, 45))
                .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 65)))
                .rounding(Rounding::same(12.0))
                .inner_margin(Margin::same(20.0));

            card_frame.show(ui, |ui| {
                ui.set_width(card_width);

                // Chart header
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Power & Torque Curves")
                            .size(18.0)
                            .strong()
                            .color(Color32::WHITE),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Legend (responsive)
                        if card_width > 600.0 {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("Torque")
                                        .size(12.0)
                                        .color(Color32::LIGHT_GRAY),
                                );
                                ui.label(
                                    RichText::new("●")
                                        .size(14.0)
                                        .color(Color32::from_rgb(70, 130, 255)),
                                );
                                ui.add_space(15.0);
                                ui.label(
                                    RichText::new("Horsepower")
                                        .size(12.0)
                                        .color(Color32::LIGHT_GRAY),
                                );
                                ui.label(
                                    RichText::new("●")
                                        .size(14.0)
                                        .color(Color32::from_rgb(255, 70, 70)),
                                );
                            });
                        }
                    });
                });

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(15.0);

                // Chart area - responsive sizing
                ui.vertical_centered(|ui| {
                    ui.allocate_ui(Vec2::new(card_width - 40.0, 300.0), |ui| {
                        self.chart.show(ui, &data.power_curve, &data.torque_curve);
                    });
                });
            });
        });
    }

    fn show_performance_cards(&self, ui: &mut egui::Ui, _data: &DynoData) {
        // Calculate responsive layout for performance cards
        let available_width = ui.available_width();
        let card_width = 200.0;
        let spacing = 20.0;
        let total_cards_width = (card_width * 2.0) + spacing;

        // Center the performance cards
        ui.horizontal(|ui| {
            let padding = (available_width - total_cards_width).max(0.0) / 2.0;
            ui.add_space(padding);

            if available_width > total_cards_width + 40.0 {
                // Wide layout - horizontal
                ui.horizontal(|ui| {
                    // Peak Horsepower card
                    self.show_performance_value_card(
                        ui,
                        "Peak Horsepower",
                        "125.7",
                        "HP",
                        Color32::from_rgb(220, 80, 80),
                        Color32::from_rgb(255, 100, 100),
                    );

                    ui.add_space(spacing);

                    // Peak Torque card
                    self.show_performance_value_card(
                        ui,
                        "Peak Torque",
                        "98.3",
                        "Nm",
                        Color32::from_rgb(70, 130, 220),
                        Color32::from_rgb(90, 150, 255),
                    );
                });
            } else {
                // Narrow layout - vertical
                ui.vertical_centered(|ui| {
                    // Peak Horsepower card
                    self.show_performance_value_card(
                        ui,
                        "Peak Horsepower",
                        "125.7",
                        "HP",
                        Color32::from_rgb(220, 80, 80),
                        Color32::from_rgb(255, 100, 100),
                    );

                    ui.add_space(spacing);

                    // Peak Torque card
                    self.show_performance_value_card(
                        ui,
                        "Peak Torque",
                        "98.3",
                        "Nm",
                        Color32::from_rgb(70, 130, 220),
                        Color32::from_rgb(90, 150, 255),
                    );
                });
            }
        });
    }

    fn show_performance_value_card(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        value: &str,
        unit: &str,
        bg_color: Color32,
        accent_color: Color32,
    ) {
        let card_frame = Frame::none()
            .fill(bg_color)
            .stroke(Stroke::NONE)
            .rounding(Rounding::same(15.0))
            .inner_margin(Margin::same(25.0));

        card_frame.show(ui, |ui| {
            ui.set_min_size(Vec2::new(200.0, 140.0));

            ui.vertical(|ui| {
                // Progress indicator at top
                let progress_rect = ui.allocate_space(Vec2::new(150.0, 6.0)).1;
                ui.painter()
                    .rect_filled(progress_rect, Rounding::same(3.0), accent_color);

                ui.add_space(20.0);

                // Title
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new(title)
                            .size(14.0)
                            .color(Color32::WHITE)
                            .strong(),
                    );
                });

                ui.add_space(15.0);

                // Large value display
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(value)
                                .size(42.0)
                                .strong()
                                .color(Color32::WHITE),
                        );
                        ui.add_space(8.0);
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                            ui.add_space(8.0); // Align with baseline
                            ui.label(
                                RichText::new(unit)
                                    .size(20.0)
                                    .color(Color32::from_rgba_premultiplied(255, 255, 255, 180)),
                            );
                        });
                    });
                });
            });
        });
    }

    // Toggle methods for panel visibility
    pub fn toggle_top_panel(&mut self) {
        self.show_top_panel = !self.show_top_panel;
    }

    pub fn toggle_left_panel(&mut self) {
        self.show_left_panel = !self.show_left_panel;
    }

    pub fn toggle_right_panel(&mut self) {
        self.show_right_panel = !self.show_right_panel;
    }

    pub fn toggle_bottom_panel(&mut self) {
        self.show_bottom_panel = !self.show_bottom_panel;
    }

    pub fn reset_layout(&mut self) {
        self.show_top_panel = true;
        self.show_left_panel = true;
        self.show_right_panel = true;
        self.show_bottom_panel = true;
    }

    // Panel visibility getters
    pub fn is_top_panel_visible(&self) -> bool {
        self.show_top_panel
    }

    pub fn is_left_panel_visible(&self) -> bool {
        self.show_left_panel
    }

    pub fn is_right_panel_visible(&self) -> bool {
        self.show_right_panel
    }

    pub fn is_bottom_panel_visible(&self) -> bool {
        self.show_bottom_panel
    }

    // Sidebar delegation methods
    pub fn selected_item(&self) -> SidebarItem {
        self.sidebar.selected_item().clone()
    }

    // Right panel section methods
    fn show_live_data_section(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        ui.vertical(|ui| {
            // Header
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("Live Data").size(18.0).color(Color32::WHITE));
                ui.add_space(15.0);
            });

            // SENSORS section
            ui.label(
                RichText::new("SENSORS")
                    .size(12.0)
                    .strong()
                    .color(Color32::LIGHT_GRAY),
            );
            ui.add_space(8.0);

            // Create sensor cards grid
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
            Frame::none()
                .fill(Color32::from_rgb(40, 40, 40))
                .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 60)))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::symmetric(15.0, 12.0))
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

            ui.add_space(20.0);

            // Peak Values section
            ui.vertical(|ui| {
                // Peak Horsepower card
                self.large_metric_card(
                    ui,
                    "Peak Horsepower",
                    &format!("{:.1}", data.peak_hp),
                    "HP",
                    Color32::from_rgb(180, 50, 50),
                );

                ui.add_space(12.0);

                // Peak Torque card
                self.large_metric_card(
                    ui,
                    "Peak Torque",
                    &format!("{:.1}", data.peak_torque),
                    "Nm",
                    Color32::from_rgb(50, 100, 180),
                );
            });
        });
    }

    fn show_test_information_section(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        ui.vertical(|ui| {
            // Test Information header
            ui.label(
                RichText::new("Test Information")
                    .size(16.0)
                    .strong()
                    .color(Color32::WHITE),
            );
            ui.add_space(8.0);

            // Test info items
            self.test_info_item(ui, "Run ID:", &format!("#{:03}", data.run_id));
            self.test_info_item(ui, "Date:", "2024-07-26");
            self.test_info_item(ui, "Motorcycle:", "Yamaha R1");
            self.test_info_item(ui, "Time:", "00:02:10");
        });
    }

    // Helper methods for right panel components
    fn sensor_card(&self, ui: &mut egui::Ui, label: &str, unit_suffix: &str, value: &str) {
        let width = 70.0;
        let height = 50.0;

        Frame::none()
            .fill(Color32::from_rgb(35, 35, 35))
            .stroke(Stroke::new(1.0, Color32::from_rgb(55, 55, 55)))
            .rounding(Rounding::same(6.0))
            .inner_margin(Margin::symmetric(8.0, 6.0))
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

        Frame::none()
            .fill(color.gamma_multiply(0.3))
            .stroke(Stroke::new(2.0, color))
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::symmetric(16.0, 12.0))
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
