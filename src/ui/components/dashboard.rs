use crate::core::models::{Customer, Motorcycle, Operator, TestParameters, TestSession};
use crate::ui::components::{CircularGauge, ValueDisplay, PerformanceGraph};
use crate::utils::formatting;
use eframe::egui;

/// Main dashboard component for authenticated operators
pub struct Dashboard {
    current_operator: Operator,
    selected_tab: DashboardTab,
    customers: Vec<Customer>,
    selected_customer: Option<Customer>,
    selected_motorcycle: Option<Motorcycle>,
    current_test_session: Option<TestSession>,
    test_parameters: TestParameters,
    is_testing: bool,
    performance_graph: PerformanceGraph,
}

#[derive(Debug, Clone, PartialEq)]
enum DashboardTab {
    Testing,
    History,
    Settings,
}

impl Dashboard {
    /// Create a new dashboard for the authenticated operator
    pub fn new(operator: Operator) -> Self {
        Self {
            current_operator: operator,
            selected_tab: DashboardTab::Testing,
            customers: Self::load_mock_customers(), // TODO: Load from actual data source
            selected_customer: None,
            selected_motorcycle: None,
            current_test_session: None,
            test_parameters: TestParameters::default(),
            is_testing: false,
            performance_graph: PerformanceGraph::new(800.0, 200.0),
        }
    }

    /// Show the dashboard UI and return true if user wants to logout
    pub fn show(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_logout = false;

        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show_inside(ui, |ui| {
            should_logout = self.render_top_menu(ui);
        });

        // Side panel for navigation
        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(200.0)
            .show_inside(ui, |ui| {
                self.render_navigation(ui);
            });

        // Main content area
        egui::CentralPanel::default().show_inside(ui, |ui| match self.selected_tab {
            DashboardTab::Testing => self.render_testing_panel(ui),
            DashboardTab::History => self.render_history_panel(ui),
            DashboardTab::Settings => self.render_settings_panel(ui),
        });

        should_logout
    }

    /// Render the top menu bar
    fn render_top_menu(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_logout = false;

        ui.horizontal(|ui| {
            ui.heading("🏍 Motorcycle Dyno Test System");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Logout button
                if ui.button("🚪 Logout").clicked() {
                    should_logout = true;
                }

                ui.separator();

                // Operator info
                ui.label(format!("👤 {}", self.current_operator.name));

                ui.separator();

                // Status indicator
                let status_text = if self.is_testing {
                    "🔴 Testing in Progress"
                } else {
                    "🟢 Ready"
                };
                ui.label(status_text);
            });
        });

        should_logout
    }

    /// Render the navigation sidebar
    fn render_navigation(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Navigation");
            ui.separator();

            // Testing tab
            let testing_button =
                ui.selectable_label(self.selected_tab == DashboardTab::Testing, "🧪 Testing");
            if testing_button.clicked() {
                self.selected_tab = DashboardTab::Testing;
            }

            // History tab
            let history_button = ui.selectable_label(
                self.selected_tab == DashboardTab::History,
                "📊 Test History",
            );
            if history_button.clicked() {
                self.selected_tab = DashboardTab::History;
            }

            // Settings tab
            let settings_button =
                ui.selectable_label(self.selected_tab == DashboardTab::Settings, "⚙ Settings");
            if settings_button.clicked() {
                self.selected_tab = DashboardTab::Settings;
            }

            ui.separator();

            // Quick info panel
            ui.heading("Quick Info");
            ui.label(format!("Operator: {}", self.current_operator.username));
            if let Some(customer) = &self.selected_customer {
                ui.label(format!("Customer: {}", customer.name));
            }
            if let Some(motorcycle) = &self.selected_motorcycle {
                ui.label(format!(
                    "Motorcycle: {} {}",
                    motorcycle.make, motorcycle.model
                ));
            }
        });
    }

    /// Render the testing panel
    fn render_testing_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Left panel - Customer and motorcycle selection
            ui.vertical(|ui| {
                ui.set_width(300.0);
                self.render_customer_selection(ui);
                ui.separator();
                self.render_motorcycle_selection(ui);
            });

            ui.separator();

            // Right panel - Real-time testing interface
            ui.vertical(|ui| {
                self.render_testing_interface(ui);
            });
        });
    }

    /// Render customer selection
    fn render_customer_selection(&mut self, ui: &mut egui::Ui) {
        ui.heading("Customer Selection");

        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for customer in &self.customers {
                    let is_selected = self
                        .selected_customer
                        .as_ref()
                        .map(|c| c.id == customer.id)
                        .unwrap_or(false);

                    if ui.selectable_label(is_selected, &customer.name).clicked() {
                        self.selected_customer = Some(customer.clone());
                        self.selected_motorcycle = None; // Reset motorcycle selection
                    }
                }
            });
    }

    /// Render motorcycle selection
    fn render_motorcycle_selection(&mut self, ui: &mut egui::Ui) {
        ui.heading("Motorcycle Selection");

        if let Some(customer) = &self.selected_customer {
            egui::ScrollArea::vertical()
                .max_height(150.0)
                .show(ui, |ui| {
                    for motorcycle in &customer.motorcycles {
                        let is_selected = self
                            .selected_motorcycle
                            .as_ref()
                            .map(|m| m.id == motorcycle.id)
                            .unwrap_or(false);

                        let label = format!(
                            "{} {} ({})",
                            motorcycle.make, motorcycle.model, motorcycle.year
                        );
                        if ui.selectable_label(is_selected, label).clicked() {
                            self.selected_motorcycle = Some(motorcycle.clone());
                        }
                    }
                });
        } else {
            ui.label("Please select a customer first");
        }
    }

    /// Render the testing interface
    fn render_testing_interface(&mut self, ui: &mut egui::Ui) {
        ui.heading("Real-time Testing");

        if self.selected_customer.is_some() && self.selected_motorcycle.is_some() {
            // Test controls
            ui.horizontal(|ui| {
                let start_button_text = if self.is_testing {
                    "Stop Test"
                } else {
                    "Start Test"
                };
                if ui.button(start_button_text).clicked() {
                    self.is_testing = !self.is_testing;
                }

                if ui.button("Reset").clicked() {
                    self.test_parameters = TestParameters::default();
                    self.performance_graph.clear();
                }
            });

            ui.separator();

            // Top row: RPM and Speed gauges side by side
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    [ui.available_width() / 2.0, 200.0].into(),
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        CircularGauge::rpm(self.test_parameters.rpm).show(ui);
                    },
                );

                ui.allocate_ui_with_layout(
                    [ui.available_width(), 200.0].into(),
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        CircularGauge::speed(self.test_parameters.speed_kmh).show(ui);
                    },
                );
            });

            ui.add_space(20.0);

            // Middle row: Torque and HP values side by side
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    [ui.available_width() / 2.0, 120.0].into(),
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        ValueDisplay::torque(self.test_parameters.torque_nm, None).show(ui);
                    },
                );

                ui.allocate_ui_with_layout(
                    [ui.available_width(), 120.0].into(),
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        ValueDisplay::power(self.test_parameters.power_hp, None).show(ui);
                    },
                );
            });

            ui.add_space(20.0);

            // Bottom row: Performance graph
            ui.vertical_centered(|ui| {
                ui.heading("Performance Graph");
                ui.separator();
                
                // Show the actual performance graph
                self.performance_graph.show(ui);
            });

            ui.add_space(10.0);

            // Additional parameters in compact form
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Engine Temp:");
                    ui.label(formatting::format_temperature(
                        self.test_parameters.engine_temp_c,
                    ));
                });

                ui.separator();

                ui.vertical(|ui| {
                    ui.label("Air/Fuel Ratio:");
                    ui.label(formatting::format_afr(self.test_parameters.air_fuel_ratio));
                });
            });

            // Simulate real-time data updates when testing
            if self.is_testing {
                self.simulate_test_data();
            }
        } else {
            ui.vertical_centered(|ui| {
                ui.label("Please select a customer and motorcycle to begin testing");
            });
        }
    }

    /// Render the history panel
    fn render_history_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Test History");
        ui.separator();

        ui.label("Previous test sessions will be displayed here");
        // TODO: Implement test history display
    }

    /// Render the settings panel
    fn render_settings_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.separator();

        ui.label("Application settings and preferences will be available here");
        // TODO: Implement settings panel
    }

    /// Simulate test data for demonstration
    fn simulate_test_data(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f32();

        // Simulate realistic dyno test data
        self.test_parameters.rpm = (3000.0 + 2000.0 * (time * 0.5).sin()).max(0.0);
        self.test_parameters.power_hp = (50.0 + 30.0 * (time * 0.3).sin()).max(0.0);
        self.test_parameters.torque_nm = (80.0 + 20.0 * (time * 0.4).sin()).max(0.0);
        self.test_parameters.engine_temp_c = 85.0 + 10.0 * (time * 0.1).sin();
        self.test_parameters.air_fuel_ratio = 14.7 + 0.5 * (time * 0.2).sin();
        
        // Calculate speed from RPM using realistic motorcycle parameters
        // Assuming: 6th gear ratio ~1.0, final drive ~2.5, wheel circumference ~2.0m
        let gear_ratio = 1.0;
        let final_drive_ratio = 2.5;
        let wheel_circumference_m = 2.0;
        
        // Speed (km/h) = (RPM * wheel_circumference * 60) / (gear_ratio * final_drive_ratio * 1000)
        self.test_parameters.speed_kmh = (self.test_parameters.rpm * wheel_circumference_m * 60.0) 
            / (gear_ratio * final_drive_ratio * 1000.0);
        
        // Add data point to performance graph
        self.performance_graph.add_data_point(
            self.test_parameters.rpm,
            self.test_parameters.torque_nm,
            self.test_parameters.power_hp,
        );
    }

    /// Load mock customer data for demonstration
    fn load_mock_customers() -> Vec<Customer> {
        vec![
            Customer {
                id: 1,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                phone: Some("+1234567890".to_string()),
                motorcycles: vec![Motorcycle {
                    id: 1,
                    make: "Honda".to_string(),
                    model: "CBR600RR".to_string(),
                    year: 2022,
                    engine_cc: 600,
                    vin: Some("JH2PC4000XM000001".to_string()),
                }],
            },
            Customer {
                id: 2,
                name: "Jane Smith".to_string(),
                email: "jane@example.com".to_string(),
                phone: Some("+1234567891".to_string()),
                motorcycles: vec![
                    Motorcycle {
                        id: 2,
                        make: "Yamaha".to_string(),
                        model: "YZF-R1".to_string(),
                        year: 2023,
                        engine_cc: 1000,
                        vin: Some("JYARN23E0XA000001".to_string()),
                    },
                    Motorcycle {
                        id: 3,
                        make: "Kawasaki".to_string(),
                        model: "Ninja ZX-10R".to_string(),
                        year: 2021,
                        engine_cc: 1000,
                        vin: Some("JKBZXNC1XBA000001".to_string()),
                    },
                ],
            },
        ]
    }
}
