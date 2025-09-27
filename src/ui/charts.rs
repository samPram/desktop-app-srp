use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2, Rounding};

pub struct PowerTorqueChart {
    width: f32,
    height: f32,
}

impl PowerTorqueChart {
    pub fn new() -> Self {
        Self {
            width: 600.0,
            height: 350.0,
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        power_curve: &[(f32, f32)], // (RPM, HP)
        torque_curve: &[(f32, f32)], // (RPM, Nm)
    ) {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let chart_rect = response.rect;
        let margin_left = 60.0;  // More space for HP labels
        let margin_right = 60.0; // Space for Torque labels
        let margin_top = 40.0;
        let margin_bottom = 60.0; // Space for RPM labels

        let plot_rect = Rect::from_min_max(
            chart_rect.min + Vec2::new(margin_left, margin_top),
            chart_rect.max - Vec2::new(margin_right, margin_bottom),
        );

        // Modern background with gradient effect
        self.draw_background(&painter, chart_rect);

        // Determine RPM range for X-axis
        let mut max_rpm = 12000.0; // Default 12000 RPM
        let mut min_rpm = 1000.0;  // Default 1000 RPM
        if !power_curve.is_empty() || !torque_curve.is_empty() {
            let all_rpms: Vec<f32> = power_curve.iter().chain(torque_curve.iter())
                .map(|(rpm, _)| *rpm).collect();
            if !all_rpms.is_empty() {
                min_rpm = all_rpms.iter().fold(f32::INFINITY, |a, &b| a.min(b)).max(500.0f32);
                max_rpm = all_rpms.iter().fold(0.0f32, |a, &b| a.max(b)).max(12000.0f32);
            }
        }

        // Grid with better styling
        self.draw_grid(&painter, plot_rect);

        // Enhanced axes
        self.draw_axes(&painter, plot_rect, chart_rect, min_rpm, max_rpm);

        // HP curve (red) - referenced to left Y-axis
        if !power_curve.is_empty() {
            self.draw_curve(
                &painter,
                plot_rect,
                power_curve,
                Color32::from_rgb(255, 80, 80),  // Bright red
                0.0,
                200.0, // 0-200 HP range
                min_rpm,
                max_rpm, // RPM range
            );

            // Add glow effect for HP curve
            self.draw_curve_glow(
                &painter,
                plot_rect,
                power_curve,
                Color32::from_rgba_premultiplied(255, 80, 80, 60),
                0.0,
                200.0,
                min_rpm,
                max_rpm,
            );
        }

        // Torque curve (blue) - referenced to right Y-axis
        if !torque_curve.is_empty() {
            self.draw_curve(
                &painter,
                plot_rect,
                torque_curve,
                Color32::from_rgb(80, 150, 255), // Bright blue
                0.0,
                150.0, // 0-150 Nm range
                min_rpm,
                max_rpm, // RPM range
            );

            // Add glow effect for Torque curve
            self.draw_curve_glow(
                &painter,
                plot_rect,
                torque_curve,
                Color32::from_rgba_premultiplied(80, 150, 255, 60),
                0.0,
                150.0,
                min_rpm,
                max_rpm,
            );
        }

        // Enhanced legend
        self.draw_legend(&painter, chart_rect);

        // Chart title
        painter.text(
            chart_rect.center_top() + Vec2::new(0.0, 15.0),
            egui::Align2::CENTER_TOP,
            "Horsepower & Torque vs RPM",
            egui::FontId::proportional(16.0),
            Color32::WHITE,
        );
    }

    fn draw_background(&self, painter: &egui::Painter, rect: Rect) {
        // Main background with rounded corners
        painter.rect_filled(
            rect,
            Rounding::same(12.0),
            Color32::from_rgb(25, 28, 35)
        );

        // Border
        painter.rect_stroke(
            rect,
            Rounding::same(12.0),
            Stroke::new(1.5, Color32::from_rgb(60, 65, 75)),
        );

        // Inner area with slight gradient effect
        let inner_rect = rect.shrink(8.0);
        painter.rect_filled(
            inner_rect,
            Rounding::same(8.0),
            Color32::from_rgb(20, 23, 30)
        );
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: Rect) {
        let major_grid_color = Color32::from_rgb(45, 50, 60);
        let minor_grid_color = Color32::from_rgb(35, 40, 50);

        // Major vertical grid lines (every 2500 RPM)
        for i in 0..=6 {
            let x = rect.min.x + (i as f32 / 6.0) * rect.width();
            let stroke = if i % 2 == 0 {
                Stroke::new(1.0, major_grid_color)
            } else {
                Stroke::new(0.5, minor_grid_color)
            };

            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                stroke,
            );
        }

        // Major horizontal grid lines
        for i in 0..=8 {
            let y = rect.min.y + (i as f32 / 8.0) * rect.height();
            let stroke = if i % 2 == 0 {
                Stroke::new(1.0, major_grid_color)
            } else {
                Stroke::new(0.5, minor_grid_color)
            };

            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                stroke,
            );
        }
    }

    fn draw_axes(&self, painter: &egui::Painter, plot_rect: Rect, chart_rect: Rect, min_rpm: f32, max_rpm: f32) {
        let axis_color = Color32::from_rgb(180, 185, 195);
        let axis_stroke = Stroke::new(2.0, axis_color);
        let label_color = Color32::from_rgb(200, 205, 215);

        // X-axis (bottom)
        painter.line_segment(
            [plot_rect.left_bottom(), plot_rect.right_bottom()],
            axis_stroke
        );

        // Left Y-axis (HP)
        painter.line_segment(
            [plot_rect.left_bottom(), plot_rect.left_top()],
            Stroke::new(2.0, Color32::from_rgb(255, 80, 80))
        );

        // Right Y-axis (Torque)
        painter.line_segment(
            [plot_rect.right_bottom(), plot_rect.right_top()],
            Stroke::new(2.0, Color32::from_rgb(80, 150, 255))
        );

        // X-axis labels (RPM) - bottom
        for i in 0..=6 {
            let x = plot_rect.min.x + (i as f32 / 6.0) * plot_rect.width();
            let rpm = min_rpm + (i as f32 / 6.0) * (max_rpm - min_rpm); // min_rpm to max_rpm
            painter.text(
                Pos2::new(x, plot_rect.max.y + 20.0),
                egui::Align2::CENTER_TOP,
                &format!("{:.0}", rpm),
                egui::FontId::proportional(11.0),
                label_color,
            );
        }

        // Left Y-axis labels (HP)
        for i in 0..=8 {
            let y = plot_rect.max.y - (i as f32 / 8.0) * plot_rect.height();
            let hp = (i * 25) as i32; // 0 to 200 HP
            painter.text(
                Pos2::new(plot_rect.min.x - 15.0, y),
                egui::Align2::RIGHT_CENTER,
                &format!("{}", hp),
                egui::FontId::proportional(11.0),
                Color32::from_rgb(255, 120, 120),
            );
        }

        // Right Y-axis labels (Torque)
        for i in 0..=6 {
            let y = plot_rect.max.y - (i as f32 / 6.0) * plot_rect.height();
            let torque = (i * 25) as i32; // 0 to 150 Nm
            painter.text(
                Pos2::new(plot_rect.max.x + 15.0, y),
                egui::Align2::LEFT_CENTER,
                &format!("{}", torque),
                egui::FontId::proportional(11.0),
                Color32::from_rgb(120, 180, 255),
            );
        }

        // Axis titles
        painter.text(
            Pos2::new(plot_rect.center().x, chart_rect.max.y - 15.0),
            egui::Align2::CENTER_CENTER,
            "RPM",
            egui::FontId::proportional(14.0),
            Color32::WHITE,
        );

        // Rotate and position HP label (left side)
        painter.text(
            Pos2::new(15.0, plot_rect.center().y),
            egui::Align2::CENTER_CENTER,
            "HP",
            egui::FontId::proportional(14.0),
            Color32::from_rgb(255, 120, 120),
        );

        // Torque label (right side)
        painter.text(
            Pos2::new(chart_rect.max.x - 15.0, plot_rect.center().y),
            egui::Align2::CENTER_CENTER,
            "Nm",
            egui::FontId::proportional(14.0),
            Color32::from_rgb(120, 180, 255),
        );
    }

    fn draw_curve(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        curve: &[(f32, f32)],
        color: Color32,
        min_y: f32,
        max_y: f32,
        min_x: f32,
        max_x: f32,
    ) {
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

        // Draw the curve with enhanced thickness
        for i in 0..points.len() - 1 {
            painter.line_segment([points[i], points[i + 1]], Stroke::new(3.0, color));
        }

        // Add data points
        for point in &points {
            painter.circle_filled(*point, 3.0, color);
            painter.circle_stroke(*point, 3.0, Stroke::new(1.0, Color32::WHITE));
        }
    }

    fn draw_curve_glow(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        curve: &[(f32, f32)],
        glow_color: Color32,
        min_y: f32,
        max_y: f32,
        min_x: f32,
        max_x: f32,
    ) {
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

        // Draw glow effect (thicker, transparent line)
        for i in 0..points.len() - 1 {
            painter.line_segment([points[i], points[i + 1]], Stroke::new(6.0, glow_color));
        }
    }

    fn draw_legend(&self, painter: &egui::Painter, chart_rect: Rect) {
        let legend_rect = Rect::from_min_size(
            chart_rect.right_top() + Vec2::new(-180.0, 50.0),
            Vec2::new(160.0, 60.0),
        );

        // Legend background
        painter.rect_filled(
            legend_rect,
            Rounding::same(6.0),
            Color32::from_rgba_premultiplied(30, 35, 45, 200),
        );
        painter.rect_stroke(
            legend_rect,
            Rounding::same(6.0),
            Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
        );

        let legend_pos = legend_rect.min + Vec2::new(15.0, 20.0);

        // HP line
        painter.line_segment(
            [legend_pos, legend_pos + Vec2::new(25.0, 0.0)],
            Stroke::new(3.0, Color32::from_rgb(255, 80, 80)),
        );
        painter.text(
            legend_pos + Vec2::new(35.0, 0.0),
            egui::Align2::LEFT_CENTER,
            "Horsepower",
            egui::FontId::proportional(12.0),
            Color32::WHITE,
        );

        // Torque line
        let torque_pos = legend_pos + Vec2::new(0.0, 20.0);
        painter.line_segment(
            [torque_pos, torque_pos + Vec2::new(25.0, 0.0)],
            Stroke::new(3.0, Color32::from_rgb(80, 150, 255)),
        );
        painter.text(
            torque_pos + Vec2::new(35.0, 0.0),
            egui::Align2::LEFT_CENTER,
            "Torque",
            egui::FontId::proportional(12.0),
            Color32::WHITE,
        );
    }
}