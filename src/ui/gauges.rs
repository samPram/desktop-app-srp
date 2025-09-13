use egui::{Ui, Color32, Pos2, Vec2, Stroke};

pub struct Gauge {
    pub title: &'static str,
    pub value: f32,
    pub unit: &'static str,
    pub min: f32,
    pub max: f32,
    pub color: Color32,
}

impl Gauge {
    pub fn ui(&self, ui: &mut Ui, size: f32) {
        let (rect, painter) = ui.allocate_painter(Vec2::splat(size), egui::Sense::hover());

        let center = rect.rect.center();
        let radius = size / 2.0 - 10.0;

        // Lingkaran luar
        painter.circle_stroke(center, radius, Stroke::new(2.0, self.color));

        // Skala garis kecil (opsional: setiap 10%)
        for i in 0..=10 {
            let t = i as f32 / 10.0;
            let angle = std::f32::consts::PI * (1.0 + t); // setengah lingkaran bawah → atas
            let dir = Vec2::angled(angle);
            let p1 = center + dir * (radius - 8.0);
            let p2 = center + dir * (radius - 2.0);
            painter.line_segment([p1, p2], Stroke::new(1.0, Color32::GRAY));
        }

        // Jarum
        let norm = (self.value - self.min) / (self.max - self.min);
        let angle = std::f32::consts::PI * (1.0 + norm); // dari kiri (min) ke kanan (max)
        let dir = Vec2::angled(angle);
        let needle_end = center + dir * (radius - 12.0);
        painter.line_segment([center, needle_end], Stroke::new(3.0, self.color));

        // Title & value
        painter.text(
            Pos2::new(center.x, center.y - 10.0),
            egui::Align2::CENTER_BOTTOM,
            self.title,
            egui::TextStyle::Body.resolve(ui.style()),
            Color32::WHITE,
        );

        painter.text(
            Pos2::new(center.x, center.y + 20.0),
            egui::Align2::CENTER_TOP,
            format!("{:.1} {}", self.value, self.unit),
            egui::FontId::proportional(16.0),
            self.color,
        );
    }
}

pub fn gauges_ui(ui: &mut Ui, rpm: f32, speed: f32) {
    ui.horizontal(|ui| {
        let rpm_gauge = Gauge {
            title: "RPM (x1000)",
            value: rpm / 1000.0, // supaya needle di 6.8 → 6850 rpm
            unit: "k/min",
            min: 0.0,
            max: 18.0,
            color: Color32::RED,
        };
        rpm_gauge.ui(ui, 180.0);

        let speed_gauge = Gauge {
            title: "Speed",
            value: speed,
            unit: "km/h",
            min: 0.0,
            max: 300.0,
            color: Color32::BLUE,
        };
        speed_gauge.ui(ui, 180.0);
    });
}
