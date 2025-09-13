use egui::{Ui, Color32};
use egui_plot::{Plot, Line, Legend};

/// Contoh data dummy horsepower & torque vs RPM
fn sample_dyno_data() -> (Vec<[f64; 2]>, Vec<[f64; 2]>) {
    let mut hp = Vec::new();
    let mut tq = Vec::new();

    for rpm in (1000..=12000).step_by(500) {
        // Horsepower (kurva dummy: naik, lalu turun)
        let hp_val = (rpm as f64 / 1000.0) * (1.2 - (rpm as f64 / 15000.0));
        // Torque (kurva dummy: puncak di 7000 rpm)
        let tq_val = 80.0 + (-(rpm as f64 - 7000.0).powi(2) / 1_000_000.0);

        hp.push([rpm as f64, hp_val * 100.0]);
        tq.push([rpm as f64, tq_val]);
    }

    (hp, tq)
}

pub fn chart_ui(ui: &mut Ui) {
    let (hp, tq) = sample_dyno_data();

    Plot::new("dyno_chart")
        .legend(Legend::default())
        .allow_zoom(false)
        .allow_scroll(false)
        .height(300.0)
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new("Horsepower", hp).color(Color32::RED)
            );
            plot_ui.line(
                Line::new("Torque", tq).color(Color32::BLUE)
            );
        });
}
