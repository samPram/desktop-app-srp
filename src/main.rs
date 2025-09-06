mod app;
mod core;
mod ui;
mod utils;

use app::DesktopApp;

fn main() -> Result<(), eframe::Error> {
    // Configure logging
    env_logger::init();

    // Set up native options for the application window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Motorcycle Dyno Test Application"),
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Motorcycle Dyno Test",
        options,
        Box::new(|cc| Ok(Box::new(DesktopApp::new(cc)))),
    )
}
