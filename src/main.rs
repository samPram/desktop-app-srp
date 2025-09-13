mod app;
mod ui;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 820.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MotoDyno - egui Demo",
        native_options,
        Box::new(|_cc| Ok(Box::new(app::AppState::default()))),
    );
}
