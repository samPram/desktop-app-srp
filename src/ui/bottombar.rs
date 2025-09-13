use eframe::egui;

pub struct Bottombar {
    status_message: String,
    cpu_usage: String,
    ram_usage: String,
    disk_free: String,
}

impl Bottombar {
    pub fn new() -> Self {
        Self {
            status_message: "Connected to Dyno Hardware. Ready for test.".to_string(),
            cpu_usage: "15%".to_string(),
            ram_usage: "2.5GB".to_string(),
            disk_free: "45GB Free".to_string(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Bottombar with dark gray background
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(50, 50, 50)) // Dark gray background
            .show(ui, |ui| {
                ui.set_height(30.0); // Fixed height for bottombar
                
                // Horizontal layout with left and right alignment
                ui.horizontal(|ui| {
                    // Left side - Status message
                    ui.label(
                        egui::RichText::new(&format!("Status: {}", self.status_message))
                            .size(12.0)
                            .color(egui::Color32::LIGHT_GRAY)
                    );

                    // Push content to the right
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Right side - System information (right to left order)
                        ui.label(
                            egui::RichText::new(&format!("Disk {}", self.disk_free))
                                .size(12.0)
                                .color(egui::Color32::LIGHT_GRAY)
                        );

                        ui.separator();

                        ui.label(
                            egui::RichText::new(&format!("RAM {}", self.ram_usage))
                                .size(12.0)
                                .color(egui::Color32::LIGHT_GRAY)
                        );

                        ui.separator();

                        ui.label(
                            egui::RichText::new(&format!("CPU: {}", self.cpu_usage))
                                .size(12.0)
                                .color(egui::Color32::LIGHT_GRAY)
                        );
                    });
                });
            });
    }

    // Methods to update system information
    pub fn update_status(&mut self, status: String) {
        self.status_message = status;
    }

    pub fn update_system_info(&mut self, cpu: String, ram: String, disk: String) {
        self.cpu_usage = cpu;
        self.ram_usage = ram;
        self.disk_free = disk;
    }
}

impl Default for Bottombar {
    fn default() -> Self {
        Self::new()
    }
}