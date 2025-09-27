use eframe::egui;

mod app;
mod config;
mod data;
mod dyno;
mod ui;
mod utils;

fn main() -> Result<(), eframe::Error> {
    // Suppress XDG Settings Portal timeout warnings by setting environment variable
    std::env::set_var("ADWAITA_DISABLE_PORTAL", "1");
    
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "SRP Dyno Test v1.0.0",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(app::DesktopApp::new(cc)))
        }),
    )
}
