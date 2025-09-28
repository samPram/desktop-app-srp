use crate::data::models::DynoData;
use crate::data::repository::TestRepository;
use crate::data::db_models::Motorcycle;
use crate::dyno::serial_comm::{SerialConnection, ConnectionStatus};
use crate::ui::motorcycle_modal::MotorcycleModal;
use crate::ui::panels::TestState;
use eframe::egui::{self, Button, Color32, RichText, ComboBox};
use log::{info, warn, error, debug};
use std::sync::Arc;

pub struct RunControls {
    is_running: bool,
    emergency_stop: bool,
    serial_connection: Option<SerialConnection>,
    available_ports: Vec<String>,
    selected_port: String,
    simulation_mode: bool,
}

impl Default for RunControls {
    fn default() -> Self {
        Self {
            is_running: false,
            emergency_stop: false,
            serial_connection: None,
            available_ports: Vec::new(),
            selected_port: String::new(),
            simulation_mode: false,
        }
    }
}

impl RunControls {
    pub fn new() -> Self {
        let available_ports = SerialConnection::list_ports();
        let selected_port = available_ports.first().cloned().unwrap_or_default();
        
        Self {
            is_running: false,
            emergency_stop: false,
            serial_connection: Some(SerialConnection::new()),
            available_ports,
            selected_port,
            simulation_mode: false,
        }
    }

    pub fn show(
        &mut self, 
        ui: &mut egui::Ui, 
        data: &mut DynoData,
        repository: Option<Arc<TestRepository>>,
        motorcycle_modal: &mut MotorcycleModal,
        motorcycle_info: &mut Option<Motorcycle>,
        test_state: &mut TestState,
    ) {
        self.show_with_motorcycle_support(ui, data, repository, motorcycle_modal, motorcycle_info, test_state);
    }
    
    // Legacy method for classic layout compatibility
    pub fn show_classic(&mut self, ui: &mut egui::Ui, data: &mut DynoData) {
        // Create dummy variables for classic layout
        let mut dummy_modal = MotorcycleModal::new();
        let mut dummy_motorcycle = None;
        let mut dummy_test_state = TestState::NoMotorcycle;
        
        self.show_with_motorcycle_support(ui, data, None, &mut dummy_modal, &mut dummy_motorcycle, &mut dummy_test_state);
    }
    
    fn show_with_motorcycle_support(
        &mut self, 
        ui: &mut egui::Ui, 
        data: &mut DynoData,
        repository: Option<Arc<TestRepository>>,
        motorcycle_modal: &mut MotorcycleModal,
        motorcycle_info: &mut Option<Motorcycle>,
        test_state: &mut TestState,
    ) {
        ui.vertical(|ui| {
            // Connection controls row
            ui.horizontal(|ui| {
                ui.label(RichText::new("Connection:").size(12.0).color(Color32::LIGHT_GRAY));
                
                // Port selection
                ComboBox::from_label("")
                    .selected_text(&self.selected_port)
                    .width(120.0)
                    .show_ui(ui, |ui| {
                        for port in &self.available_ports {
                            ui.selectable_value(&mut self.selected_port, port.clone(), port);
                        }
                    });

                // Refresh ports button
                if ui.add(Button::new("🔄").min_size([25.0, 25.0].into())).clicked() {
                    info!("User clicked refresh ports button");
                    self.available_ports = SerialConnection::list_ports();
                    info!("Refreshed ports list: {:?}", self.available_ports);
                    if !self.available_ports.contains(&self.selected_port) {
                        let old_port = self.selected_port.clone();
                        self.selected_port = self.available_ports.first().cloned().unwrap_or_default();
                        info!("Selected port changed from '{}' to '{}'", old_port, self.selected_port);
                    }
                }

                ui.add_space(10.0);

                // Connection status and buttons
                if let Some(ref connection) = self.serial_connection {
                    match connection.get_status() {
                        ConnectionStatus::Disconnected => {
                            if ui.add(Button::new("Connect").fill(Color32::from_rgb(60, 140, 60))).clicked() {
                                info!("User clicked Connect button for port: {}", self.selected_port);
                                if let Some(ref mut conn) = self.serial_connection {
                                    match conn.connect(&self.selected_port) {
                                        Ok(()) => {
                                            info!("Connection attempt completed successfully");
                                        }
                                        Err(e) => {
                                            error!("Connection attempt failed: {}", e);
                                        }
                                    }
                                }
                            }
                            
                            ui.add_space(5.0);
                            
                            if ui.add(Button::new("Simulate").fill(Color32::from_rgb(60, 100, 180))).clicked() {
                                if let Some(ref mut conn) = self.serial_connection {
                                    conn.start_simulation();
                                    self.simulation_mode = true;
                                    data.start_simulation();
                                }
                            }
                        }
                        ConnectionStatus::Connected => {
                            if ui.add(Button::new("Disconnect").fill(Color32::from_rgb(140, 60, 60))).clicked() {
                                if let Some(ref mut conn) = self.serial_connection {
                                    if self.simulation_mode {
                                        conn.stop_simulation();
                                        self.simulation_mode = false;
                                        data.stop_simulation();
                                    } else {
                                        conn.disconnect();
                                    }
                                }
                            }
                        }
                        ConnectionStatus::Connecting => {
                            ui.label(RichText::new("Connecting...").color(Color32::YELLOW));
                        }
                        ConnectionStatus::Error(ref msg) => {
                            ui.label(RichText::new(format!("Error: {}", msg)).color(Color32::RED));
                        }
                    }
                } else {
                    ui.label(RichText::new("No connection").color(Color32::RED));
                }
            });

            ui.add_space(10.0);

            // Test controls row
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
                    self.toggle_test(data, repository, motorcycle_modal, motorcycle_info, test_state);
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
                    self.emergency_stop(data);
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
                    self.reset(data);
                }

                ui.add_space(20.0);

                // Status indicator
                let (status_text, status_color) = self.get_status_display();
                ui.label(RichText::new(status_text).size(14.0).color(status_color));

                // Mode indicator
                if self.simulation_mode {
                    ui.label(RichText::new("(SIM)").size(12.0).color(Color32::from_rgb(0, 255, 255)));
                }
            });

            ui.add_space(10.0);
        });

        // Update data from serial connection
        if let Some(ref connection) = self.serial_connection {
            let status = connection.get_status();
            data.update_connection_status(&status, self.simulation_mode);
            
            if matches!(status, ConnectionStatus::Connected) {
                let arduino_data = connection.get_data();
                data.update_from_arduino(&arduino_data);
            }
        }
    }

    pub fn is_emergency_stop(&self) -> bool {
        self.emergency_stop
    }

    pub fn reset(&mut self, data: &mut DynoData) {
        self.is_running = false;
        self.emergency_stop = false;
        data.reset_data();
    }

    fn toggle_test(
        &mut self, 
        data: &mut DynoData,
        repository: Option<Arc<TestRepository>>,
        motorcycle_modal: &mut MotorcycleModal,
        motorcycle_info: &mut Option<Motorcycle>,
        test_state: &mut TestState,
    ) {
        if !self.is_running {
            // Starting test - check if we have a motorcycle selected
            match test_state {
                TestState::NoMotorcycle => {
                    // Need to select motorcycle first
                    motorcycle_modal.open();
                    return; // Don't start test yet
                },
                TestState::MotorcycleSelected => {
                    // We have a motorcycle, start the test
                    *test_state = TestState::TestRunning;
                    self.is_running = true;
                    data.start_test();
                    log::info!("Test started for motorcycle: {:?}", motorcycle_info.as_ref().map(|m| format!("{} {}", m.brand, m.model)));
                },
                TestState::TestRunning => {
                    // This shouldn't happen when starting
                    return;
                }
            }
        } else {
            // Stopping test
            self.is_running = false;
            self.emergency_stop = false;
            data.stop_test();
            *test_state = TestState::MotorcycleSelected; // Keep motorcycle selected
            log::info!("Test stopped");
        }

        // Send commands to Arduino if connected
        if let Some(ref mut connection) = self.serial_connection {
            if matches!(connection.get_status(), ConnectionStatus::Connected) && !connection.is_simulation_mode() {
                let command = if self.is_running { "START" } else { "STOP" };
                let _ = connection.send_command(command);
            }
        }
    }

    fn emergency_stop(&mut self, data: &mut DynoData) {
        self.emergency_stop = true;
        self.is_running = false;
        data.stop_test();

        // Send emergency stop command to Arduino if connected
        if let Some(ref mut connection) = self.serial_connection {
            if matches!(connection.get_status(), ConnectionStatus::Connected) && !connection.is_simulation_mode() {
                let _ = connection.send_command("STOP");
            }
        }
    }

    fn get_status_display(&self) -> (&str, Color32) {
        if self.emergency_stop {
            ("EMERGENCY STOP ACTIVE", Color32::from_rgb(220, 60, 60))
        } else if self.is_running {
            ("Test Running", Color32::from_rgb(60, 180, 60))
        } else {
            ("Ready", Color32::from_rgb(220, 180, 60))
        }
    }

    pub fn get_connection_status(&self) -> ConnectionStatus {
        if let Some(ref connection) = self.serial_connection {
            connection.get_status()
        } else {
            ConnectionStatus::Disconnected
        }
    }

    pub fn get_connection_status_text(&self) -> String {
        match self.get_connection_status() {
            ConnectionStatus::Disconnected => "Disconnected".to_string(),
            ConnectionStatus::Connecting => "Connecting...".to_string(),
            ConnectionStatus::Connected => {
                if self.simulation_mode {
                    "Simulation Mode".to_string()
                } else {
                    "Connected to Hardware".to_string()
                }
            }
            ConnectionStatus::Error(ref msg) => format!("Error: {}", msg),
        }
    }
}
