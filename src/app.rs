use crate::core::models::Operator;
use crate::ui::components::{Dashboard, LoginForm};
use eframe::egui;

/// Application state enum to manage different views
#[derive(Debug)]
enum AppState {
    Login,
    Dashboard(Operator),
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Login
    }
}

/// Main application state and logic
pub struct DesktopApp {
    state: AppState,
    login_form: LoginForm,
    dashboard: Option<Dashboard>,
    show_about_dialog: bool,
    show_settings_dialog: bool,
}

impl Default for DesktopApp {
    fn default() -> Self {
        Self {
            state: AppState::Login,
            login_form: LoginForm::new(),
            dashboard: None,
            show_about_dialog: false,
            show_settings_dialog: false,
        }
    }
}

impl DesktopApp {
    /// Create a new instance of the application
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure egui visuals and fonts here if needed
        Self::configure_ui(&cc.egui_ctx);
        Self::default()
    }

    /// Configure UI styling and fonts
    fn configure_ui(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        // Increase default font sizes for better readability in workshop environment
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(24.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        );

        // Set larger spacing for better touch/click targets
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);

        ctx.set_style(style);
    }
}

impl DesktopApp {
    /// Render the menu bar
    fn render_menu_bar(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // File menu
                ui.menu_button("File", |ui| {
                    if ui.button("New Test Session").clicked() {
                        // TODO: Implement new test session
                        ui.close_menu();
                    }
                    if ui.button("Open Test History").clicked() {
                        // TODO: Implement open test history
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Export Results...").clicked() {
                        // TODO: Implement export functionality
                        ui.close_menu();
                    }
                    if ui.button("Print Report...").clicked() {
                        // TODO: Implement print functionality
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                // Edit menu
                ui.menu_button("Edit", |ui| {
                    if ui.button("Preferences...").clicked() {
                        self.show_settings_dialog = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Clear Test Data").clicked() {
                        // TODO: Implement clear test data
                        ui.close_menu();
                    }
                    if ui.button("Reset Settings").clicked() {
                        // TODO: Implement reset settings
                        ui.close_menu();
                    }
                });

                // View menu
                ui.menu_button("View", |ui| {
                    if ui.button("Full Screen").clicked() {
                        // TODO: Toggle fullscreen
                        ui.close_menu();
                    }
                    if ui.button("Zoom In").clicked() {
                        // TODO: Implement zoom in
                        ui.close_menu();
                    }
                    if ui.button("Zoom Out").clicked() {
                        // TODO: Implement zoom out
                        ui.close_menu();
                    }
                    if ui.button("Reset Zoom").clicked() {
                        // TODO: Implement reset zoom
                        ui.close_menu();
                    }
                });

                // Tools menu
                ui.menu_button("Tools", |ui| {
                    if ui.button("Calibrate Sensors").clicked() {
                        // TODO: Implement sensor calibration
                        ui.close_menu();
                    }
                    if ui.button("Test Connection").clicked() {
                        // TODO: Implement connection test
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Data Analysis").clicked() {
                        // TODO: Implement data analysis tools
                        ui.close_menu();
                    }
                });

                // Help menu
                ui.menu_button("Help", |ui| {
                    if ui.button("User Manual").clicked() {
                        // TODO: Open user manual
                        ui.close_menu();
                    }
                    if ui.button("Keyboard Shortcuts").clicked() {
                        // TODO: Show keyboard shortcuts
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Check for Updates").clicked() {
                        // TODO: Implement update check
                        ui.close_menu();
                    }
                    if ui.button("About").clicked() {
                        self.show_about_dialog = true;
                        ui.close_menu();
                    }
                });
            });
        });
    }

    /// Render dialog windows
    fn render_dialogs(&mut self, ctx: &egui::Context) {
        // About dialog
        if self.show_about_dialog {
            egui::Window::new("About Motorcycle Dyno Test")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("🏍 Motorcycle Dyno Test System");
                        ui.add_space(10.0);
                        ui.label("Version 0.1.0");
                        ui.label("Built with Rust and egui");
                        ui.add_space(10.0);
                        ui.label("Professional motorcycle performance testing");
                        ui.label("for workshop environments");
                        ui.add_space(15.0);
                        if ui.button("Close").clicked() {
                            self.show_about_dialog = false;
                        }
                    });
                });
        }

        // Settings dialog
        if self.show_settings_dialog {
            egui::Window::new("Preferences")
                .collapsible(false)
                .resizable(true)
                .default_size([400.0, 300.0])
                .show(ctx, |ui| {
                    ui.heading("Application Settings");
                    ui.separator();
                    
                    ui.label("Settings will be implemented here:");
                    ui.label("• Display preferences");
                    ui.label("• Sensor configuration");
                    ui.label("• Export options");
                    ui.label("• User preferences");
                    
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        if ui.button("OK").clicked() {
                            self.show_settings_dialog = false;
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_settings_dialog = false;
                        }
                    });
                });
        }
    }
}

impl eframe::App for DesktopApp {
    /// Called each frame to update the UI
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Render menu bar at the top
        self.render_menu_bar(ctx, frame);
        
        // Render dialog windows
        self.render_dialogs(ctx);

        match &mut self.state {
            AppState::Login => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if let Some(operator) = self.login_form.show(ui) {
                        // Login successful, switch to dashboard
                        self.dashboard = Some(Dashboard::new(operator.clone()));
                        self.state = AppState::Dashboard(operator);
                        self.login_form.clear();
                    }
                });
            }
            AppState::Dashboard(_operator) => {
                let mut should_logout = false;

                if let Some(dashboard) = &mut self.dashboard {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        should_logout = dashboard.show(ui);
                    });

                    // Request repaint for real-time updates when testing
                    ctx.request_repaint();
                } else {
                    // This shouldn't happen, but handle gracefully
                    eprintln!("Dashboard state without dashboard instance");
                    should_logout = true;
                }

                if should_logout {
                    // User clicked logout, return to login screen
                    self.state = AppState::Login;
                    self.dashboard = None;
                    self.login_form.clear();
                }
            }
        }
    }

    /// Called when the app is being closed
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Perform cleanup if needed
        println!("Application shutting down");
    }

    /// Save application state
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Save application preferences if needed
        if let AppState::Dashboard(operator) = &self.state {
            eframe::set_value(storage, "last_operator", &operator.username);
        }
    }
}
