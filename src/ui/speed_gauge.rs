use eframe::egui::{self, Color32, Pos2, Stroke, Vec2};
use std::f32::consts::PI;

pub struct SpeedGauge {
    pub size: f32,
}

impl SpeedGauge {
    pub fn new() -> Self {
        Self { size: 160.0 }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, speed: f32) {
        let (response, painter) =
            ui.allocate_painter(Vec2::splat(self.size + 60.0), egui::Sense::hover());

        let center = response.rect.center();
        let outer_radius = self.size * 0.45;
        let inner_radius = outer_radius * 0.75;

        // Draw background circles
        painter.circle_stroke(
            center,
            outer_radius,
            Stroke::new(2.0, Color32::from_rgb(80, 80, 80)),
        );

        painter.circle_stroke(
            center,
            inner_radius,
            Stroke::new(1.0, Color32::from_rgb(60, 60, 60)),
        );

        // Draw tick marks and numbers
        self.draw_tick_marks_and_numbers(&painter, center, outer_radius, inner_radius);

        // Draw colored zones
        self.draw_yellow_zone(&painter, center, outer_radius, 100.0, 130.0); // Yellow zone
        self.draw_red_zone(&painter, center, outer_radius, 130.0, 200.0); // Red zone

        // Draw current value arc
        let clamped_speed = speed.min(200.0).max(0.0);
        if clamped_speed > 0.0 {
            self.draw_value_arc(&painter, center, outer_radius, clamped_speed);
        }

        // Draw needle
        self.draw_needle(&painter, center, inner_radius * 0.9, clamped_speed);

        // Center hub
        painter.circle_filled(center, 12.0, Color32::from_rgb(40, 40, 40));
        painter.circle_stroke(
            center,
            12.0,
            Stroke::new(2.0, Color32::from_rgb(100, 100, 100)),
        );
        painter.circle_filled(center, 6.0, Color32::from_rgb(60, 120, 220));

        // Main display value
        painter.text(
            center + Vec2::new(0.0, 8.0),
            egui::Align2::CENTER_CENTER,
            &format!("{:.1}", speed),
            egui::FontId::proportional(26.0),
            Color32::WHITE,
        );

        // Unit text
        painter.text(
            center + Vec2::new(0.0, 28.0),
            egui::Align2::CENTER_CENTER,
            "km/h",
            egui::FontId::proportional(12.0),
            Color32::LIGHT_GRAY,
        );

        // Actual speed value below unit (same as main display in this case)
        painter.text(
            center + Vec2::new(0.0, 45.0),
            egui::Align2::CENTER_CENTER,
            &format!("{:.0}", speed),
            egui::FontId::proportional(16.0),
            Color32::from_rgb(220, 220, 220),
        );

        // Title below gauge
        painter.text(
            center + Vec2::new(0.0, outer_radius + 25.0),
            egui::Align2::CENTER_CENTER,
            "Speed (km/h)",
            egui::FontId::proportional(16.0),
            Color32::WHITE,
        );
    }

    fn draw_tick_marks_and_numbers(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        outer_radius: f32,
        inner_radius: f32,
    ) {
        let start_angle = -PI * 0.75; // Start from top-left
        let end_angle = PI * 0.75; // End at top-right
        let total_angle = end_angle - start_angle;

        // Draw major tick marks (0, 20, 40, 60, 80, 100, 120, 140, 160, 180, 200)
        for i in 0..=10 {
            let value = i * 20;
            let angle = start_angle + (i as f32 / 10.0) * total_angle;

            // Major tick mark
            let inner_pos = center
                + Vec2::new(
                    angle.cos() * (inner_radius + 5.0),
                    angle.sin() * (inner_radius + 5.0),
                );
            let outer_pos = center
                + Vec2::new(
                    angle.cos() * (outer_radius - 3.0),
                    angle.sin() * (outer_radius - 3.0),
                );

            painter.line_segment(
                [inner_pos, outer_pos],
                Stroke::new(2.0, Color32::LIGHT_GRAY),
            );

            // Number labels
            let label_pos = center
                + Vec2::new(
                    angle.cos() * (outer_radius - 20.0),
                    angle.sin() * (outer_radius - 20.0),
                );
            painter.text(
                label_pos,
                egui::Align2::CENTER_CENTER,
                &format!("{}", value),
                egui::FontId::proportional(10.0),
                Color32::WHITE,
            );
        }

        // Draw minor tick marks (10, 30, 50, 70, 90, 110, 130, 150, 170, 190)
        for i in 0..10 {
            let angle = start_angle + ((i as f32 + 0.5) / 10.0) * total_angle;

            let inner_pos = center
                + Vec2::new(
                    angle.cos() * (inner_radius + 8.0),
                    angle.sin() * (inner_radius + 8.0),
                );
            let outer_pos = center
                + Vec2::new(
                    angle.cos() * (outer_radius - 6.0),
                    angle.sin() * (outer_radius - 6.0),
                );

            painter.line_segment([inner_pos, outer_pos], Stroke::new(1.0, Color32::GRAY));
        }
    }

    fn draw_yellow_zone(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        start_value: f32,
        end_value: f32,
    ) {
        let start_angle = -PI * 0.75;
        let total_angle = PI * 1.5;

        let start_ratio = start_value / 200.0;
        let end_ratio = end_value / 200.0;

        let zone_start_angle = start_angle + start_ratio * total_angle;
        let zone_end_angle = start_angle + end_ratio * total_angle;

        // Draw yellow arc
        let steps = 20;
        let angle_step = (zone_end_angle - zone_start_angle) / steps as f32;

        for i in 0..steps {
            let angle1 = zone_start_angle + (i as f32) * angle_step;
            let angle2 = zone_start_angle + ((i + 1) as f32) * angle_step;

            let p1 =
                center + Vec2::new(angle1.cos() * (radius - 8.0), angle1.sin() * (radius - 8.0));
            let p2 =
                center + Vec2::new(angle2.cos() * (radius - 8.0), angle2.sin() * (radius - 8.0));

            painter.line_segment([p1, p2], Stroke::new(6.0, Color32::from_rgb(220, 180, 60)));
        }
    }

    fn draw_red_zone(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        start_value: f32,
        end_value: f32,
    ) {
        let start_angle = -PI * 0.75;
        let total_angle = PI * 1.5;

        let start_ratio = start_value / 200.0;
        let end_ratio = end_value / 200.0;

        let zone_start_angle = start_angle + start_ratio * total_angle;
        let zone_end_angle = start_angle + end_ratio * total_angle;

        // Draw red arc
        let steps = 30;
        let angle_step = (zone_end_angle - zone_start_angle) / steps as f32;

        for i in 0..steps {
            let angle1 = zone_start_angle + (i as f32) * angle_step;
            let angle2 = zone_start_angle + ((i + 1) as f32) * angle_step;

            let p1 =
                center + Vec2::new(angle1.cos() * (radius - 8.0), angle1.sin() * (radius - 8.0));
            let p2 =
                center + Vec2::new(angle2.cos() * (radius - 8.0), angle2.sin() * (radius - 8.0));

            painter.line_segment([p1, p2], Stroke::new(6.0, Color32::from_rgb(220, 60, 60)));
        }
    }

    fn draw_value_arc(&self, painter: &egui::Painter, center: Pos2, radius: f32, value: f32) {
        let start_angle = -PI * 0.75;
        let total_angle = PI * 1.5;
        let value_angle = start_angle + (value / 200.0) * total_angle;

        let steps = ((value / 200.0) * 50.0) as i32;
        let angle_step = (value_angle - start_angle) / steps.max(1) as f32;

        for i in 0..steps {
            let angle1 = start_angle + (i as f32) * angle_step;
            let angle2 = start_angle + ((i + 1) as f32) * angle_step;

            let p1 = center
                + Vec2::new(
                    angle1.cos() * (radius - 15.0),
                    angle1.sin() * (radius - 15.0),
                );
            let p2 = center
                + Vec2::new(
                    angle2.cos() * (radius - 15.0),
                    angle2.sin() * (radius - 15.0),
                );

            painter.line_segment([p1, p2], Stroke::new(4.0, Color32::from_rgb(100, 200, 100)));
        }
    }

    fn draw_needle(&self, painter: &egui::Painter, center: Pos2, radius: f32, value: f32) {
        let start_angle = -PI * 0.75;
        let total_angle = PI * 1.5;
        let needle_angle = start_angle + (value / 200.0) * total_angle;

        let needle_end =
            center + Vec2::new(needle_angle.cos() * radius, needle_angle.sin() * radius);

        // Needle shadow
        painter.line_segment(
            [
                center + Vec2::new(1.0, 1.0),
                needle_end + Vec2::new(1.0, 1.0),
            ],
            Stroke::new(4.0, Color32::from_rgba_premultiplied(0, 0, 0, 100)),
        );

        // Main needle
        painter.line_segment(
            [center, needle_end],
            Stroke::new(3.0, Color32::from_rgb(60, 120, 220)),
        );
    }
}
