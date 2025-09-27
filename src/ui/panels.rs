use crate::data::models::DynoData;
use crate::ui::charts::PowerTorqueChart;
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
    current_page: SidebarItem, // Add current page state
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
            current_page: SidebarItem::Dashboard, // Initialize with Dashboard
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
                        let connection_status = self.run_controls.get_connection_status_text();
                        let status_color = match self.run_controls.get_connection_status() {
                            crate::dyno::serial_comm::ConnectionStatus::Connected => Color32::from_rgb(60, 180, 60),
                            crate::dyno::serial_comm::ConnectionStatus::Connecting => Color32::from_rgb(220, 180, 60),
                            crate::dyno::serial_comm::ConnectionStatus::Error(_) => Color32::from_rgb(220, 60, 60),
                            crate::dyno::serial_comm::ConnectionStatus::Disconnected => Color32::LIGHT_GRAY,
                        };
                        ui.label(
                            RichText::new(&format!("Status: {}", connection_status))
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
                        // Handle navigation changes - update current page
                        self.current_page = selected;
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
        self.show_content_based_on_selection(ui, data);
    }

    // Content switching based on sidebar selection
    fn show_content_based_on_selection(&mut self, ui: &mut egui::Ui, data: &mut DynoData) {
        match self.current_page {
            SidebarItem::Dashboard => self.show_dashboard_content(ui, data),
            SidebarItem::DynData => self.show_dyn_data_content(ui, data),
            SidebarItem::RunHistory => self.show_run_history_content(ui, data),
            SidebarItem::Reports => self.show_reports_content(ui, data),
            SidebarItem::Configuration => self.show_configuration_content(ui, data),
        }
    }

    // Main dashboard container following egui example pattern
    fn show_dashboard_content(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Dashboard");

                ui.add_space(10.0);
            });
            
            // Use ScrollArea like the egui example - it handles overflow naturally
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Use natural layout with cross justification like the example
                ui.with_layout(
                    egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
                    |ui| {

                        // Individual Gauge Cards section - using 2 column Grid with equal widths
                        ui.vertical_centered(|ui| {
                            let available_width = ui.available_width();
                            let grid_spacing = 20.0;
                            let column_width = (available_width - grid_spacing) / 2.0;
                            
                            egui::Grid::new("gauge_cards_grid")
                                .num_columns(2)
                                .spacing([grid_spacing, 10.0]) // horizontal, vertical spacing
                                .min_col_width(column_width) // Equal column widths
                                .max_col_width(column_width) // Force exact equal widths
                                .striped(false)
                                .show(ui, |ui| {
                                    // Column 1: RPM Gauge Card
                                    self.show_rpm_gauge_card(ui, data);
                                    
                                    // Column 2: Speed Gauge Card
                                    self.show_speed_gauge_card(ui, data);
                                    
                                    ui.end_row(); // End the grid row
                                });
                        });

                        ui.add_space(20.0);

                        // Chart section
                        self.show_chart_card(ui, data);

                        ui.add_space(20.0);

                        // Performance cards section
                        self.show_performance_cards(ui, data);

                        ui.add_space(20.0);
                    },
                );
            });
        });
    }

    fn show_rpm_gauge_card(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        // RPM Gauge Card with constrained size
        let card_frame = Frame::none()
            .fill(Color32::from_rgb(40, 40, 45))
            .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 65)))
            .rounding(Rounding::same(12.0))
            .inner_margin(Margin::same(15.0));

        card_frame.show(ui, |ui| {
            // Use full available width in grid column
            ui.set_width(ui.available_width());
            ui.set_min_height(260.0);
            
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
                |ui| {
                    // Card title
                    ui.label(
                        RichText::new("ENGINE RPM")
                            .size(16.0)
                            .strong()
                            .color(Color32::WHITE),
                    );
                    ui.add_space(10.0);

                    // RPM Gauge - centered in grid cell
                    ui.vertical_centered(|ui| {
                        self.rpm_gauge.show(ui, data.rpm);
                    });
                    
                    ui.add_space(10.0);
                },
            );
        });
    }

    fn show_speed_gauge_card(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        // Speed Gauge Card with constrained size
        let card_frame = Frame::none()
            .fill(Color32::from_rgb(40, 40, 45))
            .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 65)))
            .rounding(Rounding::same(12.0))
            .inner_margin(Margin::same(15.0));

        card_frame.show(ui, |ui| {
            // Use full available width in grid column
            ui.set_width(ui.available_width());
            ui.set_min_height(260.0);
            
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
                |ui| {
                    // Card title
                    ui.label(
                        RichText::new("SPEED")
                            .size(16.0)
                            .strong()
                            .color(Color32::WHITE),
                    );
                    ui.add_space(10.0);

                    // Speed Gauge - centered in grid cell
                    ui.vertical_centered(|ui| {
                        self.speed_gauge.show(ui, data.speed_kmh);
                    });
                    
                    ui.add_space(10.0);
                },
            );
        });
    }

    fn show_chart_card(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        // Chart container following egui example pattern
        let card_frame = Frame::none()
            .fill(Color32::from_rgb(40, 40, 45))
            .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 65)))
            .rounding(Rounding::same(12.0))
            .inner_margin(Margin::same(25.0));

        card_frame.show(ui, |ui| {
            // Use natural layout with cross justification
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
                |ui| {
                    ui.add_space(10.0);

                    // Use the actual PowerTorqueChart from charts.rs
                    self.chart.show(ui, &data.power_curve, &data.torque_curve);
                    
                    ui.add_space(10.0);
                },
            );
        });
    }

    fn show_performance_cards(&self, ui: &mut egui::Ui, data: &DynoData) {
        // Use natural wrapping layout like the egui example
        ui.vertical_centered(|ui| {
            let available_width = ui.available_width();
            let grid_spacing = 20.0;
            let column_width = (available_width - grid_spacing) / 2.0;

            egui::Grid::new("performance_card_grid")
                .num_columns(2)
                .spacing([grid_spacing, 10.0]) // horizontal, vertical spacing
                .min_col_width(column_width) // Equal column widths
                .max_col_width(column_width) // Force exact equal widths
                .striped(false)
                .show(ui, |ui| {
                    // Column 1: Current Horsepower card
                    self.show_performance_value_card(
                        ui,
                        "Current Horsepower",
                        &format!("{:.1}", data.horsepower),
                        "HP",
                        Color32::from_rgb(220, 80, 80),
                        Color32::from_rgb(255, 100, 100)
                    );

                    // Column 2: Current Torque card
                    self.show_performance_value_card(
                        ui,
                        "Current Torque",
                        &format!("{:.1}", data.torque),
                        "Nm",
                        Color32::from_rgb(70, 130, 220),
                        Color32::from_rgb(90, 150, 255)
                    );

                    ui.end_row(); // End the grid row
                });
        });
    }

    fn show_performance_value_card(&self, ui: &mut egui::Ui, title: &str, value: &str, unit: &str, bg_color: Color32, accent_color: Color32) {
        // Simple card frame following egui pattern
        let card_frame = Frame::none()
            .fill(bg_color)
            .stroke(Stroke::NONE)
            .rounding(Rounding::same(15.0))
            .inner_margin(Margin::same(25.0));

        card_frame.show(ui, |ui| {
            // Use natural layout with cross justification
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center).with_cross_justify(true),
                |ui| {
                    // Progress indicator at top - natural width
                    let (progress_rect, _) = ui.allocate_exact_size(
                        Vec2::new(150.0, 6.0),
                        egui::Sense::hover()
                    );
                    ui.painter().rect_filled(
                        progress_rect,
                        Rounding::same(3.0),
                        accent_color,
                    );

                    ui.add_space(20.0);

                    // Title - will wrap if needed
                    ui.label(RichText::new(title).size(14.0).color(Color32::WHITE).strong());

                    ui.add_space(15.0);

                    // Value and unit - let them flow naturally
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(value).size(42.0).strong().color(Color32::WHITE));
                        ui.add_space(8.0);
                        ui.label(RichText::new(unit).size(20.0).color(Color32::from_rgba_premultiplied(255, 255, 255, 180)));
                    });
                },
            );
        });
    }

    // Other page content methods
    fn show_dyn_data_content(&mut self, ui: &mut egui::Ui, data: &mut DynoData) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Dyn Data");
                ui.add_space(20.0);
                ui.label("Real-time dyno data visualization and analysis");
                ui.add_space(10.0);
                ui.label("🚧 Coming Soon...");
            });
        });
    }

    fn show_run_history_content(&mut self, ui: &mut egui::Ui, _data: &mut DynoData) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Run History");
                ui.add_space(20.0);
                ui.label("View and manage previous dyno test runs");
                ui.add_space(10.0);
                ui.label("🚧 Coming Soon...");
            });
        });
    }

    fn show_reports_content(&mut self, ui: &mut egui::Ui, _data: &mut DynoData) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Reports");
                ui.add_space(20.0);
                ui.label("Generate and export dyno test reports");
                ui.add_space(10.0);
                ui.label("🚧 Coming Soon...");
            });
        });
    }

    fn show_configuration_content(&mut self, ui: &mut egui::Ui, _data: &mut DynoData) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Configuration");
                ui.add_space(20.0);
                ui.label("Dyno settings and system configuration");
                ui.add_space(10.0);
                ui.label("🚧 Coming Soon...");
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
                    "Max Horsepower",
                    &format!("{:.1}", data.peak_hp),
                    "HP",
                    Color32::from_rgb(180, 50, 50),
                );

                ui.add_space(12.0);

                // Peak Torque card
                self.large_metric_card(
                    ui,
                    "max Torque",
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
