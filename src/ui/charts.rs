use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2, Rounding, FontId};

pub struct PowerTorqueChart {
    width: f32,
    height: f32,
    max_data_points: usize,
    current_rpm: f32,
    current_hp: f32,
    current_torque: f32,
    max_hp: f32,
    max_torque: f32,
    max_hp_rpm: f32,
    max_torque_rpm: f32,
}

impl PowerTorqueChart {
    pub fn new() -> Self {
        Self {
            width: 600.0,
            height: 350.0,
            max_data_points: 300,
            current_rpm: 0.0,
            current_hp: 0.0,
            current_torque: 0.0,
            max_hp: 0.0,
            max_torque: 0.0,
            max_hp_rpm: 0.0,
            max_torque_rpm: 0.0,
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        power_curve: &[(f32, f32)], // (RPM, HP)
        torque_curve: &[(f32, f32)], // (RPM, Torque)
        current_rpm: f32,
        current_hp: f32,
        current_torque: f32,
    ) {
        // Update current values and find max values
        self.current_rpm = current_rpm;
        self.current_hp = current_hp;
        self.current_torque = current_torque;
        
        // Find max values in curves
        if let Some((rpm, hp)) = power_curve.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) {
            if *hp > self.max_hp {
                self.max_hp = *hp;
                self.max_hp_rpm = *rpm;
            }
        }
        if let Some((rpm, torque)) = torque_curve.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) {
            if *torque > self.max_torque {
                self.max_torque = *torque;
                self.max_torque_rpm = *rpm;
            }
        }
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

        // For seismograph-like display, we'll use time-based x-axis (left to right)
        // Convert RPM data to time-based progression
        let seismo_power_curve = self.convert_to_seismograph_data(power_curve);
        let seismo_torque_curve = self.convert_to_seismograph_data(torque_curve);
        
        let time_range = seismo_power_curve.len().max(seismo_torque_curve.len()) as f32;
        let max_time = time_range.max(100.0); // Minimum display width

        // Seismograph-style grid
        self.draw_seismograph_grid(&painter, plot_rect);

        // Enhanced time-based axes
        self.draw_seismograph_axes(&painter, plot_rect, chart_rect, max_time);

        // HP curve (red) - seismograph style from left to right
        if !seismo_power_curve.is_empty() {
            self.draw_seismograph_curve(
                &painter,
                plot_rect,
                &seismo_power_curve,
                Color32::from_rgb(255, 80, 80),  // Bright red
                0.0,
                200.0, // 0-200 HP range
                0.0,
                max_time,
            );

            // Add glow effect for HP curve
            self.draw_seismograph_curve_glow(
                &painter,
                plot_rect,
                &seismo_power_curve,
                Color32::from_rgba_premultiplied(255, 80, 80, 60),
                0.0,
                200.0,
                0.0,
                max_time,
            );
            
            // Draw value nodes for HP
            self.draw_value_nodes(
                &painter,
                plot_rect,
                &seismo_power_curve,
                power_curve,
                Color32::from_rgb(255, 80, 80),
                0.0,
                200.0,
                0.0,
                max_time,
                "HP",
            );
        }

        // Torque curve (blue) - seismograph style from left to right
        if !seismo_torque_curve.is_empty() {
            self.draw_seismograph_curve(
                &painter,
                plot_rect,
                &seismo_torque_curve,
                Color32::from_rgb(80, 150, 255), // Bright blue
                0.0,
                150.0, // 0-150 Nm range
                0.0,
                max_time,
            );

            // Add glow effect for Torque curve
            self.draw_seismograph_curve_glow(
                &painter,
                plot_rect,
                &seismo_torque_curve,
                Color32::from_rgba_premultiplied(80, 150, 255, 60),
                0.0,
                150.0,
                0.0,
                max_time,
            );
            
            // Draw value nodes for Torque
            self.draw_value_nodes(
                &painter,
                plot_rect,
                &seismo_torque_curve,
                torque_curve,
                Color32::from_rgb(80, 150, 255),
                0.0,
                150.0,
                0.0,
                max_time,
                "Nm",
            );
        }

        // Enhanced legend with current values and max indicators
        self.draw_enhanced_legend(&painter, chart_rect);

        // Chart title with current RPM
        painter.text(
            chart_rect.center_top() + Vec2::new(0.0, 15.0),
            egui::Align2::CENTER_TOP,
            &format!("Power & Torque Chart - Current RPM: {:.0}", self.current_rpm),
            egui::FontId::proportional(16.0),
            Color32::WHITE,
        );
        
        // Draw max value indicators
        self.draw_max_indicators(&painter, plot_rect, &seismo_power_curve, &seismo_torque_curve, max_time);
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

    // Convert RPM-based data to time-based seismograph data
    fn convert_to_seismograph_data(&self, curve: &[(f32, f32)]) -> Vec<(f32, f32)> {
        curve.iter()
            .enumerate()
            .map(|(i, (_, value))| (i as f32, *value))
            .collect()
    }

    // Draw seismograph-style grid
    fn draw_seismograph_grid(&self, painter: &egui::Painter, rect: Rect) {
        let major_grid_color = Color32::from_rgb(45, 50, 60);
        let minor_grid_color = Color32::from_rgb(35, 40, 50);

        // Vertical time lines (like seismograph paper)
        for i in 0..=20 {
            let x = rect.min.x + (i as f32 / 20.0) * rect.width();
            let stroke = if i % 5 == 0 {
                Stroke::new(1.0, major_grid_color)
            } else {
                Stroke::new(0.3, minor_grid_color)
            };

            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                stroke,
            );
        }

        // Horizontal value lines
        for i in 0..=10 {
            let y = rect.min.y + (i as f32 / 10.0) * rect.height();
            let stroke = if i % 2 == 0 {
                Stroke::new(1.0, major_grid_color)
            } else {
                Stroke::new(0.3, minor_grid_color)
            };

            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                stroke,
            );
        }
    }

    // Draw seismograph-style axes
    fn draw_seismograph_axes(&self, painter: &egui::Painter, plot_rect: Rect, chart_rect: Rect, max_time: f32) {
        let axis_color = Color32::from_rgb(180, 185, 195);
        let axis_stroke = Stroke::new(2.0, axis_color);
        let label_color = Color32::from_rgb(200, 205, 215);

        // X-axis (bottom) - Time progression
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

        // X-axis labels (Time points)
        for i in 0..=10 {
            let x = plot_rect.min.x + (i as f32 / 10.0) * plot_rect.width();
            let time_point = (i as f32 / 10.0) * max_time;
            painter.text(
                Pos2::new(x, plot_rect.max.y + 20.0),
                egui::Align2::CENTER_TOP,
                &format!("{:.0}s", time_point / 10.0), // Convert to seconds
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
            "Time Progression →",
            egui::FontId::proportional(14.0),
            Color32::WHITE,
        );

        painter.text(
            Pos2::new(15.0, plot_rect.center().y),
            egui::Align2::CENTER_CENTER,
            "HP",
            egui::FontId::proportional(14.0),
            Color32::from_rgb(255, 120, 120),
        );

        painter.text(
            Pos2::new(chart_rect.max.x - 15.0, plot_rect.center().y),
            egui::Align2::CENTER_CENTER,
            "Nm",
            egui::FontId::proportional(14.0),
            Color32::from_rgb(120, 180, 255),
        );
    }

    // Draw seismograph-style curve
    fn draw_seismograph_curve(
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
            let normalized_x = if max_x > min_x { (x - min_x) / (max_x - min_x) } else { 0.0 };
            let normalized_y = if max_y > min_y { (y - min_y) / (max_y - min_y) } else { 0.0 };

            let screen_x = rect.min.x + normalized_x * rect.width();
            let screen_y = rect.max.y - normalized_y * rect.height();

            points.push(Pos2::new(screen_x, screen_y));
        }

        // Draw the seismograph line with enhanced thickness for visibility
        for i in 0..points.len() - 1 {
            painter.line_segment([points[i], points[i + 1]], Stroke::new(2.5, color));
        }
    }

    // Draw seismograph curve with glow effect
    fn draw_seismograph_curve_glow(
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
            let normalized_x = if max_x > min_x { (x - min_x) / (max_x - min_x) } else { 0.0 };
            let normalized_y = if max_y > min_y { (y - min_y) / (max_y - min_y) } else { 0.0 };

            let screen_x = rect.min.x + normalized_x * rect.width();
            let screen_y = rect.max.y - normalized_y * rect.height();

            points.push(Pos2::new(screen_x, screen_y));
        }

        // Draw glow effect (thicker, transparent line)
        for i in 0..points.len() - 1 {
            painter.line_segment([points[i], points[i + 1]], Stroke::new(5.0, glow_color));
        }
    }

    // Draw value nodes with current value display
    fn draw_value_nodes(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        seismo_curve: &[(f32, f32)],
        original_curve: &[(f32, f32)],
        color: Color32,
        min_y: f32,
        max_y: f32,
        min_x: f32,
        max_x: f32,
        unit: &str,
    ) {
        if seismo_curve.is_empty() || original_curve.is_empty() {
            return;
        }

        // Draw nodes every few points and for current position
        let step = (seismo_curve.len() / 10).max(1);
        
        for (i, &(x, y)) in seismo_curve.iter().enumerate() {
            if i % step == 0 || i == seismo_curve.len() - 1 {
                let normalized_x = if max_x > min_x { (x - min_x) / (max_x - min_x) } else { 0.0 };
                let normalized_y = if max_y > min_y { (y - min_y) / (max_y - min_y) } else { 0.0 };

                let screen_x = rect.min.x + normalized_x * rect.width();
                let screen_y = rect.max.y - normalized_y * rect.height();
                let pos = Pos2::new(screen_x, screen_y);

                // Larger node for current position
                let is_current = i == seismo_curve.len() - 1;
                let node_size = if is_current { 6.0 } else { 4.0 };
                let node_color = if is_current { Color32::WHITE } else { color };

                // Draw node
                painter.circle_filled(pos, node_size, node_color);
                painter.circle_stroke(pos, node_size, Stroke::new(2.0, Color32::WHITE));

                // Show value on hover or for current point
                if is_current && i < original_curve.len() {
                    let (rpm, value) = original_curve[i];
                    let text = format!("{:.1}{}\nRPM: {:.0}", value, unit, rpm);
                    
                    // Background for text
                    let text_size = Vec2::new(70.0, 35.0);
                    let text_rect = Rect::from_center_size(pos + Vec2::new(0.0, -25.0), text_size);
                    
                    painter.rect_filled(
                        text_rect,
                        Rounding::same(4.0),
                        Color32::from_rgba_premultiplied(0, 0, 0, 180),
                    );
                    painter.rect_stroke(
                        text_rect,
                        Rounding::same(4.0),
                        Stroke::new(1.0, color),
                    );

                    painter.text(
                        text_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &text,
                        FontId::proportional(10.0),
                        Color32::WHITE,
                    );
                }
            }
        }
    }

    // Draw enhanced legend with current values
    fn draw_enhanced_legend(&self, painter: &egui::Painter, chart_rect: Rect) {
        let legend_rect = Rect::from_min_size(
            chart_rect.right_top() + Vec2::new(-220.0, 50.0),
            Vec2::new(150.0, 130.0),
        );

        // Legend background
        painter.rect_filled(
            legend_rect,
            Rounding::same(8.0),
            Color32::from_rgba_premultiplied(30, 35, 45, 220),
        );
        painter.rect_stroke(
            legend_rect,
            Rounding::same(8.0),
            Stroke::new(1.5, Color32::from_rgb(60, 65, 75)),
        );

        let mut y_offset = 15.0;
        let x_start = legend_rect.min.x + 15.0;

        // Current values section
        painter.text(
            Pos2::new(x_start, legend_rect.min.y + y_offset),
            egui::Align2::LEFT_TOP,
            "Current Values:",
            FontId::proportional(13.0),
            Color32::WHITE,
        );
        y_offset += 20.0;

        // HP current value
        painter.circle_filled(
            Pos2::new(x_start + 5.0, legend_rect.min.y + y_offset + 5.0),
            4.0,
            Color32::from_rgb(255, 80, 80),
        );
        painter.text(
            Pos2::new(x_start + 20.0, legend_rect.min.y + y_offset),
            egui::Align2::LEFT_TOP,
            &format!("HP: {:.1}", self.current_hp),
            FontId::proportional(11.0),
            Color32::WHITE,
        );
        y_offset += 18.0;

        // Torque current value
        painter.circle_filled(
            Pos2::new(x_start + 5.0, legend_rect.min.y + y_offset + 5.0),
            4.0,
            Color32::from_rgb(80, 150, 255),
        );
        painter.text(
            Pos2::new(x_start + 20.0, legend_rect.min.y + y_offset),
            egui::Align2::LEFT_TOP,
            &format!("Torque: {:.1} Nm", self.current_torque),
            FontId::proportional(11.0),
            Color32::WHITE,
        );
        y_offset += 20.0;

        // Max values section
        painter.text(
            Pos2::new(x_start, legend_rect.min.y + y_offset),
            egui::Align2::LEFT_TOP,
            "Peak Values:",
            FontId::proportional(13.0),
            Color32::YELLOW,
        );
        y_offset += 20.0;

        painter.text(
            Pos2::new(x_start, legend_rect.min.y + y_offset),
            egui::Align2::LEFT_TOP,
            &format!("Max HP: {:.1} @ {:.0} RPM", self.max_hp, self.max_hp_rpm),
            FontId::proportional(10.0),
            Color32::from_rgb(255, 120, 120),
        );
        y_offset += 15.0;

        painter.text(
            Pos2::new(x_start, legend_rect.min.y + y_offset),
            egui::Align2::LEFT_TOP,
            &format!("Max Torque: {:.1} @ {:.0} RPM", self.max_torque, self.max_torque_rpm),
            FontId::proportional(10.0),
            Color32::from_rgb(120, 180, 255),
        );
    }

    // Draw max value indicators on the chart
    fn draw_max_indicators(
        &self,
        painter: &egui::Painter,
        plot_rect: Rect,
        seismo_power_curve: &[(f32, f32)],
        seismo_torque_curve: &[(f32, f32)],
        max_time: f32,
    ) {
        // Find and highlight max HP point
        if let Some((max_hp_index, _)) = seismo_power_curve.iter()
            .enumerate()
            .max_by(|a, b| a.1.1.partial_cmp(&b.1.1).unwrap()) 
        {
            if max_hp_index < seismo_power_curve.len() {
                let (x, y) = seismo_power_curve[max_hp_index];
                let normalized_x = if max_time > 0.0 { x / max_time } else { 0.0 };
                let normalized_y = y / 200.0; // HP scale

                let screen_x = plot_rect.min.x + normalized_x * plot_rect.width();
                let screen_y = plot_rect.max.y - normalized_y * plot_rect.height();
                let pos = Pos2::new(screen_x, screen_y);

                // Draw max HP indicator
                painter.circle_stroke(pos, 8.0, Stroke::new(3.0, Color32::YELLOW));
                painter.text(
                    pos + Vec2::new(15.0, -10.0),
                    egui::Align2::LEFT_CENTER,
                    "MAX HP",
                    FontId::proportional(9.0),
                    Color32::YELLOW,
                );
            }
        }

        // Find and highlight max torque point
        if let Some((max_torque_index, _)) = seismo_torque_curve.iter()
            .enumerate()
            .max_by(|a, b| a.1.1.partial_cmp(&b.1.1).unwrap()) 
        {
            if max_torque_index < seismo_torque_curve.len() {
                let (x, y) = seismo_torque_curve[max_torque_index];
                let normalized_x = if max_time > 0.0 { x / max_time } else { 0.0 };
                let normalized_y = y / 150.0; // Torque scale

                let screen_x = plot_rect.min.x + normalized_x * plot_rect.width();
                let screen_y = plot_rect.max.y - normalized_y * plot_rect.height();
                let pos = Pos2::new(screen_x, screen_y);

                // Draw max torque indicator
                painter.circle_stroke(pos, 8.0, Stroke::new(3.0, Color32::YELLOW));
                painter.text(
                    pos + Vec2::new(15.0, 10.0),
                    egui::Align2::LEFT_CENTER,
                    "MAX TRQ",
                    FontId::proportional(9.0),
                    Color32::YELLOW,
                );
            }
        }
    }
}