use eframe::egui;
use std::f32::consts::PI;

/// Custom circular gauge component for displaying RPM, Speed, etc.
pub struct CircularGauge {
    pub title: String,
    pub value: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub unit: String,
    pub red_zone_start: Option<f32>, // For RPM red zone
    pub size: f32,
}

impl CircularGauge {
    /// Create a new RPM gauge with red zone
    pub fn rpm(value: f32) -> Self {
        Self {
            title: "RPM".to_string(),
            value,
            min_value: 0.0,
            max_value: 12000.0,
            unit: "RPM".to_string(),
            red_zone_start: Some(9000.0), // Red zone starts at 9000 RPM
            size: 150.0,
        }
    }

    /// Create a new speed gauge
    pub fn speed(value: f32) -> Self {
        Self {
            title: "Speed".to_string(),
            value,
            min_value: 0.0,
            max_value: 300.0,
            unit: "km/h".to_string(),
            red_zone_start: None,
            size: 150.0,
        }
    }

    /// Render the circular gauge
    pub fn show(&self, ui: &mut egui::Ui) {
        let (rect, _response) = ui.allocate_exact_size(
            egui::vec2(self.size, self.size + 40.0), // Extra space for title and value
            egui::Sense::hover(),
        );

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let center = rect.center() - egui::vec2(0.0, 20.0); // Adjust for title space
            let radius = self.size * 0.4;
            let inner_radius = radius * 0.7;

            // Draw outer circle (gauge background)
            painter.circle_stroke(
                center,
                radius,
                egui::Stroke::new(3.0, egui::Color32::GRAY),
            );

            // Draw inner circle
            painter.circle_stroke(
                center,
                inner_radius,
                egui::Stroke::new(2.0, egui::Color32::DARK_GRAY),
            );

            // Calculate angles for gauge (270 degrees total, starting from bottom left)
            let start_angle = PI * 1.25; // Start at bottom left
            let end_angle = PI * 0.25;   // End at bottom right
            let total_angle = 1.5 * PI;  // 270 degrees

            // Draw gauge scale marks
            self.draw_scale_marks(&painter, center, radius, inner_radius, start_angle, total_angle);

            // Draw red zone if specified (for RPM gauge)
            if let Some(red_start) = self.red_zone_start {
                self.draw_red_zone(&painter, center, radius, inner_radius, start_angle, total_angle, red_start);
            }

            // Draw needle
            self.draw_needle(&painter, center, inner_radius, start_angle, total_angle);

            // Draw center dot
            painter.circle_filled(center, 8.0, egui::Color32::DARK_GRAY);
            painter.circle_filled(center, 5.0, egui::Color32::WHITE);

            // Draw title above gauge
            let title_pos = egui::pos2(center.x, rect.top() + 10.0);
            painter.text(
                title_pos,
                egui::Align2::CENTER_CENTER,
                &self.title,
                egui::FontId::proportional(18.0),
                egui::Color32::WHITE,
            );

            // Draw value below gauge
            let value_text = format!("{:.0} {}", self.value, self.unit);
            let value_pos = egui::pos2(center.x, center.y + radius + 25.0);
            painter.text(
                value_pos,
                egui::Align2::CENTER_CENTER,
                &value_text,
                egui::FontId::proportional(16.0),
                egui::Color32::YELLOW,
            );
        }
    }

    /// Draw scale marks around the gauge
    fn draw_scale_marks(&self, painter: &egui::Painter, center: egui::Pos2, radius: f32, inner_radius: f32, start_angle: f32, total_angle: f32) {
        let num_major_marks = 6;
        let num_minor_marks = 30;

        // Draw major marks
        for i in 0..=num_major_marks {
            let angle = start_angle - (i as f32 / num_major_marks as f32) * total_angle;
            let outer_point = center + egui::vec2(angle.cos() * radius, angle.sin() * radius);
            let inner_point = center + egui::vec2(angle.cos() * (radius - 15.0), angle.sin() * (radius - 15.0));
            
            painter.line_segment(
                [outer_point, inner_point],
                egui::Stroke::new(2.0, egui::Color32::WHITE),
            );

            // Draw scale numbers
            let value = self.min_value + (i as f32 / num_major_marks as f32) * (self.max_value - self.min_value);
            let text_point = center + egui::vec2(angle.cos() * (radius - 25.0), angle.sin() * (radius - 25.0));
            painter.text(
                text_point,
                egui::Align2::CENTER_CENTER,
                &format!("{:.0}", value),
                egui::FontId::proportional(12.0),
                egui::Color32::LIGHT_GRAY,
            );
        }

        // Draw minor marks
        for i in 0..=num_minor_marks {
            let angle = start_angle - (i as f32 / num_minor_marks as f32) * total_angle;
            let outer_point = center + egui::vec2(angle.cos() * radius, angle.sin() * radius);
            let inner_point = center + egui::vec2(angle.cos() * (radius - 8.0), angle.sin() * (radius - 8.0));
            
            painter.line_segment(
                [outer_point, inner_point],
                egui::Stroke::new(1.0, egui::Color32::GRAY),
            );
        }
    }

    /// Draw red zone for high RPM warning
    fn draw_red_zone(&self, painter: &egui::Painter, center: egui::Pos2, radius: f32, inner_radius: f32, start_angle: f32, total_angle: f32, red_start: f32) {
        let red_start_ratio = (red_start - self.min_value) / (self.max_value - self.min_value);
        let red_start_angle = start_angle - red_start_ratio * total_angle;
        let red_end_angle = start_angle - total_angle;

        // Draw red arc
        let num_segments = 20;
        for i in 0..num_segments {
            let angle1 = red_start_angle - (i as f32 / num_segments as f32) * (red_start_angle - red_end_angle);
            let angle2 = red_start_angle - ((i + 1) as f32 / num_segments as f32) * (red_start_angle - red_end_angle);
            
            let outer1 = center + egui::vec2(angle1.cos() * radius, angle1.sin() * radius);
            let outer2 = center + egui::vec2(angle2.cos() * radius, angle2.sin() * radius);
            let inner1 = center + egui::vec2(angle1.cos() * (radius - 10.0), angle1.sin() * (radius - 10.0));
            let inner2 = center + egui::vec2(angle2.cos() * (radius - 10.0), angle2.sin() * (radius - 10.0));
            
            painter.add(egui::Shape::convex_polygon(
                vec![outer1, outer2, inner2, inner1],
                egui::Color32::from_rgb(200, 50, 50),
                egui::Stroke::NONE,
            ));
        }
    }

    /// Draw the needle pointing to current value
    fn draw_needle(&self, painter: &egui::Painter, center: egui::Pos2, inner_radius: f32, start_angle: f32, total_angle: f32) {
        let value_ratio = ((self.value - self.min_value) / (self.max_value - self.min_value)).clamp(0.0, 1.0);
        let needle_angle = start_angle - value_ratio * total_angle;
        
        let needle_length = inner_radius - 10.0;
        let needle_end = center + egui::vec2(needle_angle.cos() * needle_length, needle_angle.sin() * needle_length);
        
        // Draw needle shadow
        let shadow_offset = egui::vec2(2.0, 2.0);
        painter.line_segment(
            [center + shadow_offset, needle_end + shadow_offset],
            egui::Stroke::new(4.0, egui::Color32::from_black_alpha(100)),
        );
        
        // Draw needle
        painter.line_segment(
            [center, needle_end],
            egui::Stroke::new(3.0, egui::Color32::RED),
        );
    }
}

/// Value display component for torque and power
pub struct ValueDisplay {
    pub title: String,
    pub value: f32,
    pub unit: String,
    pub max_value: Option<f32>,
}

impl ValueDisplay {
    /// Create a new torque display
    pub fn torque(value: f32, max_value: Option<f32>) -> Self {
        Self {
            title: "Torque".to_string(),
            value,
            unit: "Nm".to_string(),
            max_value,
        }
    }

    /// Create a new power display
    pub fn power(value: f32, max_value: Option<f32>) -> Self {
        Self {
            title: "Power".to_string(),
            value,
            unit: "HP".to_string(),
            max_value,
        }
    }

    /// Render the value display
    pub fn show(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            
            // Title
            ui.heading(&self.title);
            ui.add_space(5.0);
            
            // Current value (large display)
            let value_text = format!("{:.1}", self.value);
            ui.label(egui::RichText::new(&value_text)
                .size(36.0)
                .color(egui::Color32::YELLOW));
            
            // Unit
            ui.label(egui::RichText::new(&self.unit)
                .size(18.0)
                .color(egui::Color32::LIGHT_GRAY));
            
            ui.add_space(5.0);
            
            // Max value if available
            if let Some(max) = self.max_value {
                ui.label(egui::RichText::new(format!("Max: {:.1} {}", max, self.unit))
                    .size(14.0)
                    .color(egui::Color32::GRAY));
            }
            
            ui.add_space(10.0);
        });
    }
}