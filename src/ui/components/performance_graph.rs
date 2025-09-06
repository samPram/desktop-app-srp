use eframe::egui;
use std::collections::VecDeque;

/// Data point for performance graph
#[derive(Debug, Clone)]
pub struct GraphDataPoint {
    pub rpm: f32,
    pub torque_nm: f32,
    pub power_hp: f32,
    pub timestamp: f32,
}

/// Performance graph component for displaying torque and power curves
pub struct PerformanceGraph {
    data_points: VecDeque<GraphDataPoint>,
    max_points: usize,
    width: f32,
    height: f32,
}

impl PerformanceGraph {
    /// Create a new performance graph
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            data_points: VecDeque::new(),
            max_points: 100, // Keep last 100 data points
            width,
            height,
        }
    }

    /// Add a new data point to the graph
    pub fn add_data_point(&mut self, rpm: f32, torque_nm: f32, power_hp: f32) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f32();

        let data_point = GraphDataPoint {
            rpm,
            torque_nm,
            power_hp,
            timestamp,
        };

        self.data_points.push_back(data_point);

        // Keep only the last max_points
        if self.data_points.len() > self.max_points {
            self.data_points.pop_front();
        }
    }

    /// Clear all data points
    pub fn clear(&mut self) {
        self.data_points.clear();
    }

    /// Render the performance graph
    pub fn show(&self, ui: &mut egui::Ui) {
        let (rect, _response) = ui.allocate_exact_size(
            [self.width, self.height].into(),
            egui::Sense::hover(),
        );

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw background
            painter.rect_filled(
                rect,
                egui::Rounding::same(5.0),
                egui::Color32::from_gray(25),
            );

            painter.rect_stroke(
                rect,
                egui::Rounding::same(5.0),
                egui::Stroke::new(2.0, egui::Color32::GRAY),
            );

            // Draw grid and axes
            self.draw_grid(&painter, rect);

            // Draw data if available
            if !self.data_points.is_empty() {
                self.draw_curves(&painter, rect);
            } else {
                // Show placeholder text
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Performance Graph\n(Start testing to see curves)",
                    egui::FontId::proportional(16.0),
                    egui::Color32::LIGHT_GRAY,
                );
            }

            // Draw legend
            self.draw_legend(&painter, rect);
        }
    }

    /// Draw grid lines and axes
    fn draw_grid(&self, painter: &egui::Painter, rect: egui::Rect) {
        let margin = 40.0;
        let graph_rect = egui::Rect::from_min_size(
            rect.min + egui::vec2(margin, margin),
            rect.size() - egui::vec2(margin * 2.0, margin * 2.0),
        );

        // Draw axes
        painter.line_segment(
            [graph_rect.left_bottom(), graph_rect.right_bottom()],
            egui::Stroke::new(2.0, egui::Color32::WHITE),
        );
        painter.line_segment(
            [graph_rect.left_bottom(), graph_rect.left_top()],
            egui::Stroke::new(2.0, egui::Color32::WHITE),
        );

        // Draw grid lines
        let num_h_lines = 5;
        let num_v_lines = 8;

        // Horizontal grid lines
        for i in 1..num_h_lines {
            let y = graph_rect.bottom() - (i as f32 / num_h_lines as f32) * graph_rect.height();
            painter.line_segment(
                [egui::pos2(graph_rect.left(), y), egui::pos2(graph_rect.right(), y)],
                egui::Stroke::new(1.0, egui::Color32::from_gray(60)),
            );
        }

        // Vertical grid lines
        for i in 1..num_v_lines {
            let x = graph_rect.left() + (i as f32 / num_v_lines as f32) * graph_rect.width();
            painter.line_segment(
                [egui::pos2(x, graph_rect.top()), egui::pos2(x, graph_rect.bottom())],
                egui::Stroke::new(1.0, egui::Color32::from_gray(60)),
            );
        }

        // Draw axis labels
        painter.text(
            egui::pos2(rect.center().x, rect.bottom() - 10.0),
            egui::Align2::CENTER_CENTER,
            "RPM",
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        );

        painter.text(
            egui::pos2(rect.left() + 15.0, rect.center().y),
            egui::Align2::CENTER_CENTER,
            "Torque (Nm) / Power (HP)",
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        );

        // Draw scale numbers
        for i in 0..=num_v_lines {
            let rpm = (i as f32 / num_v_lines as f32) * 12000.0;
            let x = graph_rect.left() + (i as f32 / num_v_lines as f32) * graph_rect.width();
            painter.text(
                egui::pos2(x, graph_rect.bottom() + 15.0),
                egui::Align2::CENTER_CENTER,
                &format!("{:.0}", rpm),
                egui::FontId::proportional(12.0),
                egui::Color32::LIGHT_GRAY,
            );
        }

        for i in 0..=num_h_lines {
            let value = (i as f32 / num_h_lines as f32) * 150.0;
            let y = graph_rect.bottom() - (i as f32 / num_h_lines as f32) * graph_rect.height();
            painter.text(
                egui::pos2(graph_rect.left() - 25.0, y),
                egui::Align2::CENTER_CENTER,
                &format!("{:.0}", value),
                egui::FontId::proportional(12.0),
                egui::Color32::LIGHT_GRAY,
            );
        }
    }

    /// Draw torque and power curves
    fn draw_curves(&self, painter: &egui::Painter, rect: egui::Rect) {
        let margin = 40.0;
        let graph_rect = egui::Rect::from_min_size(
            rect.min + egui::vec2(margin, margin),
            rect.size() - egui::vec2(margin * 2.0, margin * 2.0),
        );

        if self.data_points.len() < 2 {
            return;
        }

        // Find min/max values for scaling
        let max_rpm = 12000.0;
        let max_torque = 150.0;
        let max_power = 150.0;

        let mut torque_points = Vec::new();
        let mut power_points = Vec::new();

        for point in &self.data_points {
            let x = graph_rect.left() + (point.rpm / max_rpm) * graph_rect.width();
            
            let torque_y = graph_rect.bottom() - (point.torque_nm / max_torque) * graph_rect.height();
            let power_y = graph_rect.bottom() - (point.power_hp / max_power) * graph_rect.height();

            torque_points.push(egui::pos2(x, torque_y));
            power_points.push(egui::pos2(x, power_y));
        }

        // Draw torque curve (blue)
        if torque_points.len() > 1 {
            for i in 0..torque_points.len() - 1 {
                painter.line_segment(
                    [torque_points[i], torque_points[i + 1]],
                    egui::Stroke::new(3.0, egui::Color32::from_rgb(100, 150, 255)),
                );
            }
        }

        // Draw power curve (red)
        if power_points.len() > 1 {
            for i in 0..power_points.len() - 1 {
                painter.line_segment(
                    [power_points[i], power_points[i + 1]],
                    egui::Stroke::new(3.0, egui::Color32::from_rgb(255, 100, 100)),
                );
            }
        }

        // Draw current value indicators
        if let Some(last_point) = self.data_points.back() {
            let x = graph_rect.left() + (last_point.rpm / max_rpm) * graph_rect.width();
            let torque_y = graph_rect.bottom() - (last_point.torque_nm / max_torque) * graph_rect.height();
            let power_y = graph_rect.bottom() - (last_point.power_hp / max_power) * graph_rect.height();

            // Current torque point
            painter.circle_filled(
                egui::pos2(x, torque_y),
                5.0,
                egui::Color32::from_rgb(100, 150, 255),
            );

            // Current power point
            painter.circle_filled(
                egui::pos2(x, power_y),
                5.0,
                egui::Color32::from_rgb(255, 100, 100),
            );
        }
    }

    /// Draw legend
    fn draw_legend(&self, painter: &egui::Painter, rect: egui::Rect) {
        let legend_x = rect.right() - 120.0;
        let legend_y = rect.top() + 20.0;

        // Torque legend
        painter.line_segment(
            [egui::pos2(legend_x, legend_y), egui::pos2(legend_x + 20.0, legend_y)],
            egui::Stroke::new(3.0, egui::Color32::from_rgb(100, 150, 255)),
        );
        painter.text(
            egui::pos2(legend_x + 25.0, legend_y),
            egui::Align2::LEFT_CENTER,
            "Torque (Nm)",
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );

        // Power legend
        let power_y = legend_y + 20.0;
        painter.line_segment(
            [egui::pos2(legend_x, power_y), egui::pos2(legend_x + 20.0, power_y)],
            egui::Stroke::new(3.0, egui::Color32::from_rgb(255, 100, 100)),
        );
        painter.text(
            egui::pos2(legend_x + 25.0, power_y),
            egui::Align2::LEFT_CENTER,
            "Power (HP)",
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );
    }
}