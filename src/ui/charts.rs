use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

pub struct PowerTorqueChart {
    width: f32,
    height: f32,
}

impl PowerTorqueChart {
    pub fn new() -> Self {
        Self {
            width: 400.0,
            height: 250.0,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, power_curve: &[(f32, f32)], torque_curve: &[(f32, f32)]) {
        let (response, painter) = ui.allocate_painter(
            Vec2::new(self.width, self.height),
            egui::Sense::hover()
        );

        let chart_rect = response.rect;
        let margin = 40.0;
        let plot_rect = Rect::from_min_size(
            chart_rect.min + Vec2::splat(margin),
            chart_rect.size() - Vec2::splat(margin * 2.0)
        );

        // Background
        painter.rect_filled(chart_rect, 5.0, Color32::from_rgb(25, 25, 25));
        painter.rect_stroke(chart_rect, 5.0, Stroke::new(1.0, Color32::from_rgb(60, 60, 60)));

        // Grid
        self.draw_grid(&painter, plot_rect);

        // Axes
        self.draw_axes(&painter, plot_rect);

        // Power curve (red)
        if !power_curve.is_empty() {
            self.draw_curve(&painter, plot_rect, power_curve, Color32::from_rgb(220, 60, 60), 0.0, 140.0, 1000.0, 12000.0);
        }

        // Torque curve (blue)
        if !torque_curve.is_empty() {
            self.draw_curve(&painter, plot_rect, torque_curve, Color32::from_rgb(60, 120, 220), 0.0, 100.0, 1000.0, 12000.0);
        }

        // Legend
        self.draw_legend(&painter, chart_rect);
        
        // Title
        painter.text(
            chart_rect.center_top() + Vec2::new(0.0, 10.0),
            egui::Align2::CENTER_TOP,
            "Horsepower vs Torque",
            egui::FontId::proportional(14.0),
            Color32::WHITE
        );
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: Rect) {
        let grid_color = Color32::from_rgb(40, 40, 40);
        let grid_stroke = Stroke::new(0.5, grid_color);

        // Vertical grid lines (RPM)
        for i in 0..=6 {
            let x = rect.min.x + (i as f32 / 6.0) * rect.width();
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                grid_stroke
            );
        }

        // Horizontal grid lines
        for i in 0..=5 {
            let y = rect.min.y + (i as f32 / 5.0) * rect.height();
            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                grid_stroke
            );
        }
    }

    fn draw_axes(&self, painter: &egui::Painter, rect: Rect) {
        let axis_color = Color32::LIGHT_GRAY;
        let axis_stroke = Stroke::new(1.0, axis_color);

        // X-axis
        painter.line_segment(
            [rect.left_bottom(), rect.right_bottom()],
            axis_stroke
        );

        // Y-axis
        painter.line_segment(
            [rect.left_bottom(), rect.left_top()],
            axis_stroke
        );

        // X-axis labels (RPM)
        for i in 0..=6 {
            let x = rect.min.x + (i as f32 / 6.0) * rect.width();
            let rpm = 1000 + (i * 1833); // 1000 to 12000 RPM
            painter.text(
                Pos2::new(x, rect.max.y + 15.0),
                egui::Align2::CENTER_TOP,
                &format!("{}", rpm),
                egui::FontId::proportional(10.0),
                Color32::GRAY
            );
        }

        // Y-axis labels
        for i in 0..=5 {
            let y = rect.max.y - (i as f32 / 5.0) * rect.height();
            let value = i * 30; // 0 to 150
            painter.text(
                Pos2::new(rect.min.x - 25.0, y),
                egui::Align2::RIGHT_CENTER,
                &format!("{}", value),
                egui::FontId::proportional(10.0),
                Color32::GRAY
            );
        }

        // Axis titles
        painter.text(
            Pos2::new(rect.center().x, rect.max.y + 35.0),
            egui::Align2::CENTER_TOP,
            "RPM",
            egui::FontId::proportional(12.0),
            Color32::WHITE
        );
    }

    fn draw_curve(&self, painter: &egui::Painter, rect: Rect, curve: &[(f32, f32)], color: Color32, min_y: f32, max_y: f32, min_x: f32, max_x: f32) {
        if curve.len() < 2 {
            return;
        }

        let mut points = Vec::new();
        for &(x, y) in curve {
            let normalized_x = (x - min_x) / (max_x - min_x);
            let normalized_y = (y - min_y) / (max_y - min_y);
            
            let screen_x = rect.min.x + normalized_x * rect.width();
            let screen_y = rect.max.y - normalized_y * rect.height();
            
            points.push(Pos2::new(screen_x, screen_y));
        }

        // Draw the curve
        for i in 0..points.len() - 1 {
            painter.line_segment(
                [points[i], points[i + 1]],
                Stroke::new(2.5, color)
            );
        }
    }

    fn draw_legend(&self, painter: &egui::Painter, chart_rect: Rect) {
        let legend_pos = chart_rect.right_top() + Vec2::new(-120.0, 30.0);
        
        // Power line
        painter.line_segment(
            [legend_pos, legend_pos + Vec2::new(20.0, 0.0)],
            Stroke::new(2.5, Color32::from_rgb(220, 60, 60))
        );
        painter.text(
            legend_pos + Vec2::new(25.0, 0.0),
            egui::Align2::LEFT_CENTER,
            "Horsepower",
            egui::FontId::proportional(10.0),
            Color32::WHITE
        );

        // Torque line
        let torque_pos = legend_pos + Vec2::new(0.0, 15.0);
        painter.line_segment(
            [torque_pos, torque_pos + Vec2::new(20.0, 0.0)],
            Stroke::new(2.5, Color32::from_rgb(60, 120, 220))
        );
        painter.text(
            torque_pos + Vec2::new(25.0, 0.0),
            egui::Align2::LEFT_CENTER,
            "Torque",
            egui::FontId::proportional(10.0),
            Color32::WHITE
        );
    }
}