use crate::data::models::DynoData;
use crate::data::repository::TestRepository;
use crate::data::db_models::Motorcycle;
use crate::ui::charts::PowerTorqueChart;
use crate::ui::data_panel::DataPanel;
use crate::ui::gauges::CircularGauge;
use crate::ui::motorcycle_modal::{MotorcycleModal, MotorcycleInfoPanel};
use crate::ui::panels::PanelLayout;
use crate::ui::rpm_gauge::RpmGauge;
use crate::ui::run_history::RunHistory;
use crate::ui::sidebar::{Sidebar, SidebarItem};
use crate::ui::speed_gauge::SpeedGauge;
use eframe::egui::{self, Color32, RichText, Stroke};
use std::sync::Arc;

pub struct Dashboard {
    sidebar: Sidebar,
    rpm_gauge: CircularGauge,
    speed_gauge: CircularGauge,
    chart: PowerTorqueChart,
    data_panel: DataPanel,
    panel_layout: PanelLayout,
    run_history: RunHistory,
    use_panel_layout: bool,
    motorcycle_modal: MotorcycleModal,
    motorcycle_info: MotorcycleInfoPanel,
    current_motorcycle: Option<Motorcycle>,
    test_started: bool,
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
            run_history: RunHistory::new(),
            use_panel_layout: true,
            motorcycle_modal: MotorcycleModal::new(),
            motorcycle_info: MotorcycleInfoPanel::new(),
            current_motorcycle: None,
            test_started: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, data: &mut DynoData, repository: Option<Arc<TestRepository>>) {
        // Handle motorcycle modal
        if let Some(motorcycle) = self.motorcycle_modal.show_modal(ui.ctx(), repository.clone()) {
            log::info!("Motorcycle selected: {} {} {}", motorcycle.brand, motorcycle.model, motorcycle.year);
            self.current_motorcycle = Some(motorcycle.clone());
            self.motorcycle_info.set_motorcycle(Some(motorcycle));
            self.test_started = false; // Reset test state
        }

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
            self.panel_layout.show(ui, data, repository);
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
                    SidebarItem::Dashboard => self.show_dashboard_content(ui, data, repository.clone()),
                    SidebarItem::RunHistory => self.show_run_history_content(ui, repository),
                    _ => {
                        ui.centered_and_justified(|ui| {
                            ui.label(RichText::new("Feature coming soon...").size(18.0));
                        });
                    }
                });
            });
        }
    }

    fn show_dashboard_content(&mut self, ui: &mut egui::Ui, data: &DynoData, _repository: Option<Arc<TestRepository>>) {
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
                            self.chart.show(
                                ui, 
                                &data.power_curve, 
                                &data.torque_curve,
                                data.rpm,
                                data.horsepower,
                                data.torque,
                            );
                        });
                    });
                });
            });

            ui.add_space(10.0);
            ui.separator();

            // Right section - motorcycle info and data panel
            ui.vertical(|ui| {
                ui.add_space(10.0);
                
                // Motorcycle info panel
                self.motorcycle_info.show(ui);
                
                ui.add_space(15.0);
                
                // Test control button
                if self.current_motorcycle.is_none() {
                    ui.vertical_centered(|ui| {
                        let start_button = ui.add_sized(
                            [200.0, 40.0],
                            egui::Button::new(RichText::new("Start Test").size(16.0))
                                .fill(Color32::from_rgb(60, 120, 60))
                        );
                        
                        if start_button.clicked() {
                            self.motorcycle_modal.open();
                        }
                        
                        ui.add_space(5.0);
                        ui.small("Select a motorcycle to begin testing");
                    });
                } else if !self.test_started {
                    ui.vertical_centered(|ui| {
                        let start_button = ui.add_sized(
                            [200.0, 40.0],
                            egui::Button::new(RichText::new("Begin Test").size(16.0))
                                .fill(Color32::from_rgb(60, 120, 60))
                        );
                        
                        if start_button.clicked() {
                            self.test_started = true;
                            log::info!("Test started for motorcycle: {:?}", self.current_motorcycle.as_ref().map(|m| format!("{} {}", m.brand, m.model)));
                        }
                        
                        ui.add_space(10.0);
                        
                        let change_button = ui.add_sized(
                            [150.0, 30.0],
                            egui::Button::new("Change Motorcycle")
                        );
                        
                        if change_button.clicked() {
                            self.current_motorcycle = None;
                            self.motorcycle_info.set_motorcycle(None);
                            self.test_started = false;
                        }
                    });
                } else {
                    ui.vertical_centered(|ui| {
                        ui.colored_label(Color32::GREEN, "● Test in progress");
                        
                        ui.add_space(10.0);
                        
                        let stop_button = ui.add_sized(
                            [200.0, 40.0],
                            egui::Button::new(RichText::new("Stop Test").size(16.0))
                                .fill(Color32::from_rgb(180, 60, 60))
                        );
                        
                        if stop_button.clicked() {
                            self.test_started = false;
                            log::info!("Test stopped");
                        }
                        
                        ui.add_space(10.0);
                        
                        let change_button = ui.add_sized(
                            [150.0, 30.0],
                            egui::Button::new("Change Motorcycle")
                        );
                        
                        if change_button.clicked() {
                            self.current_motorcycle = None;
                            self.motorcycle_info.set_motorcycle(None);
                            self.test_started = false;
                        }
                    });
                }
                
                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);
                
                // Data panel below motorcycle info
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

    fn show_run_history_content(&mut self, ui: &mut egui::Ui, repository: Option<Arc<TestRepository>>) {
        // Run History page - use full width without data panel
        ui.set_min_height(ui.available_height());
        
        // Add some padding
        ui.add_space(10.0);
        
        // Show run history component
        self.run_history.show(ui, repository);
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
    pub fn is_test_started(&self) -> bool {
        self.test_started
    }
}
