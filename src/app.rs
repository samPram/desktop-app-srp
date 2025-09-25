use eframe::egui;
use crate::ui::dashboard::Dashboard;
use crate::data::models::DynoData;

pub struct DesktopApp {
    dashboard: Dashboard,
    dyno_data: DynoData,
}

impl DesktopApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            dashboard: Dashboard::new(),
            dyno_data: DynoData::new(),
        }
    }
}

impl eframe::App for DesktopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update dyno data (simulate real-time updates)
        self.dyno_data.update();

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("View", |ui| {
                    if ui.button(if self.dashboard.is_using_panel_layout() { 
                        "Switch to Classic Layout" 
                    } else { 
                        "Switch to Panel Layout" 
                    }).clicked() {
                        self.dashboard.toggle_panel_layout();
                        ui.close_menu();
                    }

                    if self.dashboard.is_using_panel_layout() {
                        ui.separator();
                        ui.label("Panel Visibility:");
                        
                        let mut top_visible = self.dashboard.is_top_panel_visible();
                        if ui.checkbox(&mut top_visible, "Top Panel").clicked() {
                            self.dashboard.toggle_top_panel();
                        }
                        
                        let mut left_visible = self.dashboard.is_left_panel_visible();
                        if ui.checkbox(&mut left_visible, "Left Panel").clicked() {
                            self.dashboard.toggle_left_panel();
                        }
                        
                        let mut right_visible = self.dashboard.is_right_panel_visible();
                        if ui.checkbox(&mut right_visible, "Right Panel").clicked() {
                            self.dashboard.toggle_right_panel();
                        }
                        
                        let mut bottom_visible = self.dashboard.is_bottom_panel_visible();
                        if ui.checkbox(&mut bottom_visible, "Bottom Panel").clicked() {
                            self.dashboard.toggle_bottom_panel();
                        }
                        
                        ui.separator();
                        if ui.button("Reset Layout").clicked() {
                            self.dashboard.reset_panel_layout();
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button("Tools", |ui| {
                    ui.label("Coming soon...");
                });

                ui.menu_button("Help", |ui| {
                    ui.label("MotoDyno v1.0.0");
                    ui.separator();
                    ui.label("© 2024 Dyno Systems");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.dashboard.show(ui, &mut self.dyno_data);
        });

        // Request repaint for continuous updates
        ctx.request_repaint();
    }
}