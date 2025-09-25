use eframe::egui::{self, Color32, RichText, Stroke};
use crate::ui::gauges::CircularGauge;
use crate::ui::charts::PowerTorqueChart;
use crate::ui::data_panel::DataPanel;
use crate::ui::run_controls::RunControls;
use crate::ui::sidebar::{Sidebar, SidebarItem};
use crate::data::models::DynoData;

pub struct PanelLayout {
    rpm_gauge: CircularGauge,
    speed_gauge: CircularGauge,
    chart: PowerTorqueChart,
    data_panel: DataPanel,
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
            rpm_gauge: CircularGauge::new("RPM", 0.0, 12000.0, "RPM", Color32::from_rgb(220, 60, 60)),
            speed_gauge: CircularGauge::new("Speed (km/h)", 0.0, 300.0, "km/h", Color32::from_rgb(60, 120, 220)),
            chart: PowerTorqueChart::new(),
            data_panel: DataPanel::new(),
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
                .frame(egui::Frame::none()
                    .fill(Color32::from_rgb(35, 35, 35))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(50, 50, 50)))
                )
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.vertical_centered(|ui| {
                            ui.heading(RichText::new("Dyno Control Panel").size(20.0).color(Color32::WHITE));
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
                .frame(egui::Frame::none()
                    .fill(Color32::from_rgb(25, 25, 25))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(50, 50, 50)))
                )
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.label(RichText::new("Status: Connected to Dyno Hardware. Ready for test.")
                            .size(11.0).color(Color32::LIGHT_GRAY));
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(10.0);
                            ui.label(RichText::new("Disk: 94.4% Free").size(11.0).color(Color32::LIGHT_GRAY));
                            ui.separator();
                            ui.label(RichText::new("RAM: 2.5GB").size(11.0).color(Color32::LIGHT_GRAY));
                            ui.separator();
                            ui.label(RichText::new("CPU: 15%").size(11.0).color(Color32::LIGHT_GRAY));
                        });
                    });
                });
        }

        // Left panel - Navigation Sidebar
        if self.show_left_panel {
            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(200.0)
                .width_range(150.0..=250.0)
                .frame(egui::Frame::none()
                    .fill(Color32::from_rgb(35, 35, 35))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(50, 50, 50)))
                )
                .show_inside(ui, |ui| {
                    self.sidebar.show(ui);
                });
        }

        // Right panel - Data display
        if self.show_right_panel {
            egui::SidePanel::right("right_panel")
                .resizable(true)
                .default_width(250.0)
                .width_range(200.0..=300.0)
                .frame(egui::Frame::none()
                    .fill(Color32::from_rgb(30, 30, 30))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(50, 50, 50)))
                )
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(10.0);
                            ui.heading(RichText::new("Data").size(18.0).color(Color32::WHITE));
                            ui.add_space(10.0);
                        });
                        self.data_panel.show(ui, data);
                    });
                });
        }

        // Central panel - Main content area based on sidebar selection
        egui::CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(Color32::from_rgb(25, 25, 25))
                .stroke(Stroke::new(1.0, Color32::from_rgb(50, 50, 50)))
            )
            .show_inside(ui, |ui| {
                match self.sidebar.selected_item() {
                    SidebarItem::Dashboard => self.show_dashboard_content(ui, data),
                    _ => {
                        ui.centered_and_justified(|ui| {
                            ui.label(RichText::new("Feature coming soon...").size(18.0));
                        });
                    }
                }
            });
    }

    fn show_dashboard_content(&mut self, ui: &mut egui::Ui, data: &DynoData) {
        ui.vertical(|ui| {
            // Gauges section
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                
                // Left gauge column
                ui.vertical(|ui| {
                    ui.add_space(20.0);
                    ui.heading(RichText::new("RPM").size(16.0).color(Color32::WHITE));
                    self.rpm_gauge.show(ui, data.rpm);
                });
                
                ui.add_space(50.0);
                
                // Right gauge column  
                ui.vertical(|ui| {
                    ui.add_space(20.0);
                    ui.heading(RichText::new("Speed").size(16.0).color(Color32::WHITE));
                    self.speed_gauge.show(ui, data.speed_kmh);
                });
                
                ui.add_space(20.0);
            });

            ui.add_space(30.0);

            // Chart section
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("Power & Torque Chart").size(18.0).color(Color32::WHITE));
                ui.add_space(20.0);
                
                ui.horizontal_centered(|ui| {
                    self.chart.show(ui, &data.power_curve, &data.torque_curve);
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
}