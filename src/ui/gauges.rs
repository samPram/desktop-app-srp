use eframe::egui::{self, Color32, Pos2, Stroke, Vec2};
use std::f32::consts::PI;

pub struct CircularGauge {
    pub title: String,
    pub value: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub unit: String,
    pub color: Color32,
    pub size: f32,
}

impl CircularGauge {
    pub fn new(title: &str, min_value: f32, max_value: f32, unit: &str, color: Color32) -> Self {
        Self {
            title: title.to_string(),
            value: 0.0,
            min_value,
            max_value,
            unit: unit.to_string(),
            color,
            size: 140.0,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, value: f32) {
        self.value = value;

        let (response, painter) =
            ui.allocate_painter(Vec2::splat(self.size + 40.0), egui::Sense::hover());

        let center = response.rect.center();
        let radius = self.size * 0.4;
        let inner_radius = radius * 0.7;

        // Background circle
        painter.circle_stroke(
            center,
            radius,
            Stroke::new(3.0, Color32::from_rgb(60, 60, 60)),
        );

        // Value arc calculation
        let normalized_value = (self.value - self.min_value) / (self.max_value - self.min_value);
        let angle_range = PI * 1.5; // 270 degrees
        let start_angle = -PI * 0.75; // Start from top-left
        let end_angle = start_angle + (normalized_value * angle_range);

        // Draw the value arc
        if normalized_value > 0.0 {
            self.draw_arc(
                &painter,
                center,
                radius,
                start_angle,
                end_angle,
                self.color,
                4.0,
            );
        }

        // Draw tick marks
        self.draw_tick_marks(&painter, center, radius);

        // Draw needle
        self.draw_needle(&painter, center, inner_radius, end_angle);

        // Center circle
        painter.circle_filled(center, 8.0, Color32::from_rgb(40, 40, 40));
        painter.circle_filled(center, 5.0, self.color);

        // Value text in center
        let value_text = if self.value >= 1000.0 {
            format!("{:.0}", self.value)
        } else {
            format!("{:.1}", self.value)
        };

        painter.text(
            center + Vec2::new(0.0, 15.0),
            egui::Align2::CENTER_CENTER,
            &value_text,
            egui::FontId::proportional(24.0),
            Color32::WHITE,
        );

        // Unit text
        painter.text(
            center + Vec2::new(0.0, 35.0),
            egui::Align2::CENTER_CENTER,
            &self.unit,
            egui::FontId::proportional(12.0),
            Color32::LIGHT_GRAY,
        );

        // Title text below gauge
        painter.text(
            center + Vec2::new(0.0, radius + 20.0),
            egui::Align2::CENTER_CENTER,
            &self.title,
            egui::FontId::proportional(14.0),
            Color32::WHITE,
        );

        // Min/Max labels
        let min_pos = center + Vec2::new(-radius * 0.8, radius * 0.4);
        let max_pos = center + Vec2::new(radius * 0.8, radius * 0.4);

        painter.text(
            min_pos,
            egui::Align2::CENTER_CENTER,
            &format!("{}", self.min_value as i32),
            egui::FontId::proportional(10.0),
            Color32::GRAY,
        );

        painter.text(
            max_pos,
            egui::Align2::CENTER_CENTER,
            &format!("{}", self.max_value as i32),
            egui::FontId::proportional(10.0),
            Color32::GRAY,
        );
    }

    fn draw_arc(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        color: Color32,
        width: f32,
    ) {
        let steps = 50;
        let angle_step = (end_angle - start_angle) / steps as f32;

        for i in 0..steps {
            let angle1 = start_angle + (i as f32) * angle_step;
            let angle2 = start_angle + ((i + 1) as f32) * angle_step;

            let p1 = center + Vec2::new(angle1.cos() * radius, angle1.sin() * radius);
            let p2 = center + Vec2::new(angle2.cos() * radius, angle2.sin() * radius);

            painter.line_segment([p1, p2], Stroke::new(width, color));
        }
    }

    fn draw_tick_marks(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        let tick_count = 8;
        let angle_range = PI * 1.5;
        let start_angle = -PI * 0.75;

        for i in 0..=tick_count {
            let angle = start_angle + (i as f32 / tick_count as f32) * angle_range;
            let inner_pos =
                center + Vec2::new(angle.cos() * (radius - 10.0), angle.sin() * (radius - 10.0));
            let outer_pos = center + Vec2::new(angle.cos() * radius, angle.sin() * radius);

            painter.line_segment([inner_pos, outer_pos], Stroke::new(1.0, Color32::GRAY));
        }
    }

    fn draw_needle(&self, painter: &egui::Painter, center: Pos2, radius: f32, angle: f32) {
        let needle_end = center + Vec2::new(angle.cos() * radius, angle.sin() * radius);
        painter.line_segment([center, needle_end], Stroke::new(3.0, Color32::WHITE));
    }
}
