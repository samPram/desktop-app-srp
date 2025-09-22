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

        egui::CentralPanel::default().show(ctx, |ui| {
            self.dashboard.show(ui, &mut self.dyno_data);
        });

        // Request repaint for continuous updates
        ctx.request_repaint();
    }
}