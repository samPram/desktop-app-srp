use eframe::{egui, App, Frame};
use crate::ui;


pub struct AppState {
    pub rpm: f32,
    pub speed: f32,
    // nanti tambahkan: hp_curve, tq_curve, sensors, run_info, theme, dsb.
    pub active_menu: Menu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Dashboard,
    DynData,
    Cepols,
    RunHistory,
    Runs,
    Reports,
    Configuration,
}


impl Default for AppState {
    fn default() -> Self {
        Self {
            rpm: 6850.0,
            speed: 95.2,
            active_menu: Menu::Dashboard,
        }
    }
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Sidebar
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui::sidebar::sidebar_ui(ui, self);
            });

        // Topbar
        egui::TopBottomPanel::top("topbar")
            .resizable(false)
            .exact_height(40.0)
            .show(ctx, |ui| {
                ui::topbar::topbar_ui(ui);
            });

        // Bottom Status Bar
        egui::TopBottomPanel::bottom("status_bar")
            .resizable(false)
            .exact_height(30.0)
            .show(ctx, |ui| {
                ui::status_bar::status_bar_ui(ui);
            });

        // Central Dashboard
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dashboard Testing");
            ui.separator();
            ui.add_space(8.0);

            ui::cards::peak_cards_ui(ui);
            ui.add_space(16.0);

            ui::gauges::gauges_ui(ui, self.rpm, self.speed);
            ui.add_space(16.0);

            ui::chart::chart_ui(ui);
        });

        // Info Panel kanan
        egui::SidePanel::right("info_panel")
            .resizable(false)
            .default_width(220.0)
            .show(ctx, |ui| {
                ui::info_panel::info_panel_ui(ui);
            });


        // Repaint periodically so values can be updated later (simulated live)
        ctx.request_repaint_after(std::time::Duration::from_millis(1000));
    }
}
