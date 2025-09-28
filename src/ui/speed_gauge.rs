use eframe::egui::{self, Color32, Pos2, RichText, Stroke, Vec2, Rounding};
use std::f32::consts::PI;

pub struct SpeedGauge {
    pub size: f32,
}

impl SpeedGauge {
    pub fn new() -> Self {
        Self { size: 400.0 }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, speed_kmh: f32) {
        let (response, painter) =
            ui.allocate_painter(Vec2::splat(self.size + 120.0), egui::Sense::hover());

        let center = response.rect.center();
        let outer_radius = self.size * 0.42;

        // Modern color palette for speed gauge
        let bg_gradient_outer = Color32::from_rgb(10, 15, 25);
        let bg_gradient_inner = Color32::from_rgb(20, 25, 40);
        let outer_ring = Color32::from_rgb(70, 85, 120);
        let inner_ring = Color32::from_rgb(40, 50, 80);
        let major_tick_color = Color32::from_rgb(200, 220, 255);
        let minor_tick_color = Color32::from_rgb(120, 140, 180);
        let number_color = Color32::WHITE;
        let blue_zone_color = Color32::from_rgb(60, 150, 255);
        let yellow_zone_color = Color32::from_rgb(255, 200, 60);
        let red_zone_color = Color32::from_rgb(255, 80, 80);
        let progress_color = Color32::from_rgb(60, 180, 255);
        let needle_color = Color32::from_rgb(255, 120, 60);
        let glow_color = Color32::from_rgba_premultiplied(60, 180, 255, 100);

        // Multi-layer background for depth
        self.draw_gradient_background(&painter, center, outer_radius, bg_gradient_outer, bg_gradient_inner);

        // Outer decorative rings
        painter.circle_stroke(center, outer_radius + 6.0, Stroke::new(2.0, Color32::from_rgb(50, 70, 100)));
        painter.circle_stroke(center, outer_radius + 3.0, Stroke::new(1.0, outer_ring));
        painter.circle_stroke(center, outer_radius, Stroke::new(2.5, Color32::from_rgb(90, 110, 140)));

        // Inner ring for style
        painter.circle_stroke(center, outer_radius * 0.7, Stroke::new(1.5, inner_ring));

        // Draw beautiful tick marks and numbers
        self.draw_premium_ticks_and_numbers(&painter, center, outer_radius, major_tick_color, minor_tick_color, number_color);

        // Multi-zone coloring for speed ranges
        self.draw_speed_zones(&painter, center, outer_radius, blue_zone_color, yellow_zone_color, red_zone_color);

        // Current speed progress arc with glow effect
        if speed_kmh > 0.0 {
            self.draw_glowing_progress_arc(&painter, center, outer_radius, speed_kmh.min(200.0), glow_color, progress_color);
        }

        // Premium needle with gradient effect
        self.draw_premium_needle(&painter, center, outer_radius * 0.8, speed_kmh.min(200.0), needle_color);

        // Center hub with multiple rings
        self.draw_center_hub(&painter, center);

        // Digital display area with glass effect
        self.draw_digital_display(&painter, center, speed_kmh);

        // Bottom label with style
        // self.draw_bottom_label(&painter, center, outer_radius);

        // Speed indicators and warnings
        self.draw_speed_indicators(&painter, center, outer_radius, speed_kmh);
    }

    fn draw_gradient_background(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        outer_color: Color32,
        inner_color: Color32,
    ) {
        // Create gradient effect with multiple circles
        for i in 0..20 {
            let r = radius * (1.0 - i as f32 * 0.05);
            let alpha = 1.0 - (i as f32 * 0.05);
            let color = Color32::from_rgba_premultiplied(
                (outer_color.r() as f32 + (inner_color.r() as f32 - outer_color.r() as f32) * (i as f32 / 20.0)) as u8,
                (outer_color.g() as f32 + (inner_color.g() as f32 - outer_color.g() as f32) * (i as f32 / 20.0)) as u8,
                (outer_color.b() as f32 + (inner_color.b() as f32 - outer_color.b() as f32) * (i as f32 / 20.0)) as u8,
                (255.0 * alpha) as u8,
            );
            painter.circle_filled(center, r, color);
        }
    }

    fn draw_premium_ticks_and_numbers(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        outer_radius: f32,
        major_color: Color32,
        minor_color: Color32,
        number_color: Color32,
    ) {
        let start_angle = -PI * 0.75;
        let end_angle = PI * 0.75;
        let total_angle = end_angle - start_angle;

        // Major ticks and numbers (0, 20, 40, 60, 80, 100, 120, 140, 160, 180, 200)
        for i in 0..=10 {
            let value = i * 20;
            let angle = start_angle + (i as f32 / 10.0) * total_angle;

            // Major tick with gradient effect
            let tick_inner = center + Vec2::new(
                angle.cos() * (outer_radius - 35.0),
                angle.sin() * (outer_radius - 35.0),
            );
            let tick_outer = center + Vec2::new(
                angle.cos() * (outer_radius - 8.0),
                angle.sin() * (outer_radius - 8.0),
            );

            // Draw tick with glow effect
            painter.line_segment([tick_inner, tick_outer], Stroke::new(4.0, Color32::from_rgba_premultiplied(major_color.r(), major_color.g(), major_color.b(), 60)));
            painter.line_segment([tick_inner, tick_outer], Stroke::new(2.5, major_color));

            // Numbers with better typography
            let number_pos = center + Vec2::new(
                angle.cos() * (outer_radius - 45.0),
                angle.sin() * (outer_radius - 45.0),
            );

            let font_size = if i % 2 == 0 { 15.0 } else { 13.0 };
            painter.text(
                number_pos,
                egui::Align2::CENTER_CENTER,
                &format!("{}", value),
                egui::FontId::proportional(font_size),
                number_color,
            );
        }

        // Minor ticks for precision (every 10 km/h)
        for i in 0..20 { // 20 intervals of 10 km/h each
            if i % 2 != 0 { // Skip major tick positions
                let angle = start_angle + (i as f32 / 20.0) * total_angle;

                let tick_inner = center + Vec2::new(
                    angle.cos() * (outer_radius - 25.0),
                    angle.sin() * (outer_radius - 25.0),
                );
                let tick_outer = center + Vec2::new(
                    angle.cos() * (outer_radius - 12.0),
                    angle.sin() * (outer_radius - 12.0),
                );

                painter.line_segment([tick_inner, tick_outer], Stroke::new(1.5, minor_color));
            }
        }

        // Very small ticks for 5 km/h intervals
        for i in 0..40 {
            if i % 4 != 0 { // Skip major and minor tick positions
                let angle = start_angle + (i as f32 / 40.0) * total_angle;

                let tick_inner = center + Vec2::new(
                    angle.cos() * (outer_radius - 18.0),
                    angle.sin() * (outer_radius - 18.0),
                );
                let tick_outer = center + Vec2::new(
                    angle.cos() * (outer_radius - 12.0),
                    angle.sin() * (outer_radius - 12.0),
                );

                painter.line_segment([tick_inner, tick_outer], Stroke::new(0.8, Color32::from_rgb(100, 120, 160)));
            }
        }
    }

    fn draw_speed_zones(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        outer_radius: f32,
        blue_color: Color32,
        yellow_color: Color32,
        red_color: Color32,
    ) {
        let start_angle = -PI * 0.75;
        let total_angle = PI * 1.5;

        // Blue zone: 0-60 km/h (City/Safe speeds)
        self.draw_zone_arc(painter, center, outer_radius, start_angle, total_angle, 0.0, 60.0, 200.0, blue_color, 5.0);

        // Yellow zone: 60-120 km/h (Highway speeds)
        self.draw_zone_arc(painter, center, outer_radius, start_angle, total_angle, 60.0, 120.0, 200.0, yellow_color, 5.0);

        // Red zone: 120-200 km/h (High speeds)
        self.draw_zone_arc(painter, center, outer_radius, start_angle, total_angle, 120.0, 200.0, 200.0, red_color, 5.0);
    }

    fn draw_zone_arc(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        start_angle: f32,
        total_angle: f32,
        zone_start: f32,
        zone_end: f32,
        max_value: f32,
        color: Color32,
        thickness: f32,
    ) {
        let zone_start_angle = start_angle + (zone_start / max_value) * total_angle;
        let zone_end_angle = start_angle + (zone_end / max_value) * total_angle;

        let steps = 30;
        let angle_step = (zone_end_angle - zone_start_angle) / steps as f32;

        for i in 0..steps {
            let angle1 = zone_start_angle + (i as f32) * angle_step;
            let angle2 = zone_start_angle + ((i + 1) as f32) * angle_step;

            let p1 = center + Vec2::new(
                angle1.cos() * (radius - 8.0),
                angle1.sin() * (radius - 8.0),
            );
            let p2 = center + Vec2::new(
                angle2.cos() * (radius - 8.0),
                angle2.sin() * (radius - 8.0),
            );

            painter.line_segment([p1, p2], Stroke::new(thickness, color));
        }
    }

    fn draw_glowing_progress_arc(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        outer_radius: f32,
        speed: f32,
        glow_color: Color32,
        arc_color: Color32,
    ) {
        let start_angle = -PI * 0.75;
        let total_angle = PI * 1.5;
        let speed_angle = start_angle + (speed / 200.0) * total_angle;

        let steps = ((speed / 200.0) * 60.0) as i32;
        if steps > 0 {
            let angle_step = (speed_angle - start_angle) / steps as f32;

            for i in 0..steps {
                let angle1 = start_angle + (i as f32) * angle_step;
                let angle2 = start_angle + ((i + 1) as f32) * angle_step;

                let p1 = center + Vec2::new(
                    angle1.cos() * (outer_radius - 20.0),
                    angle1.sin() * (outer_radius - 20.0),
                );
                let p2 = center + Vec2::new(
                    angle2.cos() * (outer_radius - 20.0),
                    angle2.sin() * (outer_radius - 20.0),
                );

                // Glow effect
                painter.line_segment([p1, p2], Stroke::new(8.0, glow_color));
                // Main arc
                painter.line_segment([p1, p2], Stroke::new(4.0, arc_color));
            }
        }
    }

    fn draw_premium_needle(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        needle_length: f32,
        speed: f32,
        needle_color: Color32,
    ) {
        let start_angle = -PI * 0.75;
        let total_angle = PI * 1.5;
        let needle_angle = start_angle + (speed / 200.0) * total_angle;

        let needle_tip = center + Vec2::new(
            needle_angle.cos() * needle_length,
            needle_angle.sin() * needle_length,
        );

        // Needle shadow
        painter.line_segment(
            [center + Vec2::new(2.0, 2.0), needle_tip + Vec2::new(2.0, 2.0)],
            Stroke::new(6.0, Color32::from_rgba_premultiplied(0, 0, 0, 100)),
        );

        // Needle glow
        painter.line_segment(
            [center, needle_tip],
            Stroke::new(6.0, Color32::from_rgba_premultiplied(needle_color.r(), needle_color.g(), needle_color.b(), 80)),
        );

        // Main needle
        painter.line_segment([center, needle_tip], Stroke::new(3.0, needle_color));

        // Needle tip dot
        painter.circle_filled(needle_tip, 4.0, needle_color);
        painter.circle_filled(needle_tip, 2.0, Color32::WHITE);
    }

    fn draw_center_hub(&self, painter: &egui::Painter, center: Pos2) {
        // Multiple concentric circles for premium look
        painter.circle_filled(center, 16.0, Color32::from_rgb(15, 20, 35));
        painter.circle_stroke(center, 16.0, Stroke::new(2.0, Color32::from_rgb(70, 85, 120)));
        painter.circle_filled(center, 12.0, Color32::from_rgb(25, 35, 55));
        painter.circle_stroke(center, 12.0, Stroke::new(1.5, Color32::from_rgb(90, 110, 140)));
        painter.circle_filled(center, 6.0, Color32::from_rgb(10, 15, 30));
        painter.circle_filled(center, 3.0, Color32::from_rgb(255, 120, 60));
    }

    fn draw_digital_display(&self, painter: &egui::Painter, center: Pos2, speed: f32) {
        // Glass-like digital display background
        let display_rect = egui::Rect::from_center_size(
            center + Vec2::new(0.0, -15.0),
            Vec2::new(130.0, 55.0),
        );

        painter.rect_filled(
            display_rect,
            Rounding::same(8.0),
            Color32::from_rgba_premultiplied(5, 10, 20, 220),
        );
        painter.rect_stroke(
            display_rect,
            Rounding::same(8.0),
            Stroke::new(1.0, Color32::from_rgb(70, 85, 120)),
        );

        // Main speed display
        painter.text(
            center + Vec2::new(0.0, -25.0),
            egui::Align2::CENTER_CENTER,
            &format!("{:.0}", speed),
            egui::FontId::proportional(32.0),
            Color32::from_rgb(60, 180, 255),
        );

        // Unit label
        painter.text(
            center + Vec2::new(0.0, -5.0),
            egui::Align2::CENTER_CENTER,
            "km/h",
            egui::FontId::proportional(12.0),
            Color32::from_rgb(140, 160, 200),
        );

        // Speed in mph for reference
        let mph = speed * 0.621371;
        painter.text(
            center + Vec2::new(0.0, 25.0),
            egui::Align2::CENTER_CENTER,
            &format!("{:.0} mph", mph),
            egui::FontId::proportional(20.0),
            Color32::from_rgb(180, 190, 210),
        );
    }

    fn draw_bottom_label(&self, painter: &egui::Painter, center: Pos2, outer_radius: f32) {
        painter.text(
            center + Vec2::new(0.0, outer_radius + 45.0),
            egui::Align2::CENTER_CENTER,
            "SPEED",
            egui::FontId::proportional(14.0),
            Color32::from_rgb(140, 160, 200),
        );
    }

    fn draw_speed_indicators(&self, painter: &egui::Painter, center: Pos2, outer_radius: f32, speed: f32) {
        // Left indicator - Speed limit warning
        let left_pos = center + Vec2::new(-outer_radius * 0.6, outer_radius * 0.7);
        let limit_color = if speed > 50.0 {
            Color32::from_rgb(60, 180, 255)
        } else {
            Color32::from_rgb(80, 90, 110)
        };

        painter.circle_filled(left_pos, 6.0, limit_color);
        painter.text(
            left_pos + Vec2::new(0.0, 15.0),
            egui::Align2::CENTER_CENTER,
            "CRUISE",
            egui::FontId::proportional(10.0),
            Color32::from_rgb(140, 160, 200),
        );

        // Right indicator - High speed warning
        let right_pos = center + Vec2::new(outer_radius * 0.6, outer_radius * 0.7);
        let high_speed_color = if speed > 120.0 {
            Color32::from_rgb(255, 80, 80)
        } else if speed > 80.0 {
            Color32::from_rgb(255, 200, 60)
        } else {
            Color32::from_rgb(80, 90, 110)
        };

        painter.circle_filled(right_pos, 6.0, high_speed_color);
        painter.text(
            right_pos + Vec2::new(0.0, 15.0),
            egui::Align2::CENTER_CENTER,
            "FAST",
            egui::FontId::proportional(10.0),
            Color32::from_rgb(140, 160, 200),
        );

        // Center bottom - Current gear or mode indicator
        let mode_pos = center + Vec2::new(0.0, outer_radius * 0.8);
        painter.circle_filled(mode_pos, 8.0, Color32::from_rgb(25, 35, 55));
        painter.circle_stroke(mode_pos, 8.0, Stroke::new(1.0, Color32::from_rgb(70, 85, 120)));

        let gear = match speed {
            s if s < 10.0 => "N",
            s if s < 30.0 => "1",
            s if s < 60.0 => "2",
            s if s < 90.0 => "3",
            s if s < 120.0 => "4",
            s if s < 160.0 => "5",
            _ => "6",
        };

        painter.text(
            mode_pos,
            egui::Align2::CENTER_CENTER,
            gear,
            egui::FontId::proportional(12.0),
            Color32::WHITE,
        );
    }
}