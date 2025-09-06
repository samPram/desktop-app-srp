use crate::core::models::Operator;
use eframe::egui;

/// Login form component for operator authentication
#[derive(Default)]
pub struct LoginForm {
    username: String,
    password: String,
    remember_me: bool,
    login_error: Option<String>,
    is_logging_in: bool,
}

impl LoginForm {
    /// Create a new login form
    pub fn new() -> Self {
        Self::default()
    }

    /// Show the login form and return the authenticated operator if login is successful
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<Operator> {
        let mut authenticated_operator = None;

        ui.vertical_centered(|ui| {
            ui.add_space(50.0);

            // Application title
            ui.heading("Motorcycle Dyno Test System");
            ui.add_space(30.0);

            // Login form container
            egui::Frame::none()
                .fill(ui.style().visuals.faint_bg_color)
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(20.0))
                .show(ui, |ui| {
                    ui.set_max_width(350.0);

                    ui.vertical_centered(|ui| {
                        ui.heading("Operator Login");
                        ui.add_space(20.0);

                        // Username field
                        ui.horizontal(|ui| {
                            ui.label("Username:");
                            ui.add_space(10.0);
                        });
                        let username_response = ui.add(
                            egui::TextEdit::singleline(&mut self.username)
                                .desired_width(300.0)
                                .hint_text("Enter your username"),
                        );

                        ui.add_space(10.0);

                        // Password field
                        ui.horizontal(|ui| {
                            ui.label("Password:");
                            ui.add_space(10.0);
                        });
                        let password_response = ui.add(
                            egui::TextEdit::singleline(&mut self.password)
                                .password(true)
                                .desired_width(300.0)
                                .hint_text("Enter your password"),
                        );

                        ui.add_space(15.0);

                        // Remember me checkbox
                        ui.checkbox(&mut self.remember_me, "Remember me");

                        ui.add_space(20.0);

                        // Login button
                        let login_button = ui.add_enabled(
                            !self.username.is_empty()
                                && !self.password.is_empty()
                                && !self.is_logging_in,
                            egui::Button::new(if self.is_logging_in {
                                "Logging in..."
                            } else {
                                "Login"
                            })
                            .min_size(egui::vec2(120.0, 35.0)),
                        );

                        // Handle Enter key press
                        if (username_response.lost_focus()
                            && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                            || (password_response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                            || login_button.clicked()
                        {
                            if !self.username.is_empty() && !self.password.is_empty() {
                                authenticated_operator = self.attempt_login();
                            }
                        }

                        // Show error message if login failed
                        if let Some(error) = &self.login_error {
                            ui.add_space(10.0);
                            ui.colored_label(egui::Color32::RED, error);
                        }

                        ui.add_space(10.0);

                        // Additional info
                        ui.separator();
                        ui.add_space(10.0);
                        ui.small("Contact your supervisor if you need login credentials");
                    });
                });
        });

        authenticated_operator
    }

    /// Attempt to authenticate the operator
    fn attempt_login(&mut self) -> Option<Operator> {
        self.is_logging_in = true;
        self.login_error = None;

        // TODO: Replace with actual authentication logic
        // For now, we'll use hardcoded credentials for demonstration
        let valid_operators = vec![
            ("admin", "admin123", "Administrator"),
            ("operator1", "pass123", "John Smith"),
            ("operator2", "pass456", "Jane Doe"),
            ("tech", "tech2024", "Technical Support"),
        ];

        for (username, password, name) in valid_operators {
            if self.username == username && self.password == password {
                self.is_logging_in = false;
                return Some(Operator {
                    id: 1, // TODO: Use proper ID from database
                    username: self.username.clone(),
                    name: name.to_string(),
                    is_active: true,
                });
            }
        }

        // Login failed
        self.is_logging_in = false;
        self.login_error = Some("Invalid username or password".to_string());
        None
    }

    /// Clear the form
    pub fn clear(&mut self) {
        self.username.clear();
        self.password.clear();
        self.remember_me = false;
        self.login_error = None;
        self.is_logging_in = false;
    }

    /// Check if form has any input
    pub fn has_input(&self) -> bool {
        !self.username.is_empty() || !self.password.is_empty()
    }
}
