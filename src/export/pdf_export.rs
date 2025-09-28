use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use crate::data::db_models::{TestSession, Motorcycle, Operator, Customer, TestDataPoint, PowerCurveData};
use crate::data::repository::TestRepository;
use crate::data::database::DatabaseError;

#[derive(Debug)]
pub enum ExportError {
    Database(DatabaseError),
    Pdf(String),
    Io(std::io::Error),
    Image(String),
}

impl From<DatabaseError> for ExportError {
    fn from(err: DatabaseError) -> Self {
        ExportError::Database(err)
    }
}

impl From<std::io::Error> for ExportError {
    fn from(err: std::io::Error) -> Self {
        ExportError::Io(err)
    }
}

impl From<printpdf::Error> for ExportError {
    fn from(err: printpdf::Error) -> Self {
        ExportError::Pdf(format!("PDF error: {:?}", err))
    }
}

pub struct PdfExporter {
    repository: std::sync::Arc<TestRepository>,
}

impl PdfExporter {
    pub fn new(repository: std::sync::Arc<TestRepository>) -> Self {
        Self { repository }
    }

    pub fn export_test_session(&self, session_id: i32, output_path: &str) -> Result<(), ExportError> {
        // Fetch all required data
        let sessions = self.repository.get_test_sessions(None).map_err(ExportError::from)?;
        let session = sessions.into_iter()
            .find(|s| s.test_session.id == session_id)
            .ok_or_else(|| ExportError::Pdf("Test session not found".to_string()))?;

        let test_data = self.repository.get_test_data_points(session_id)
            .map_err(ExportError::from)?;

        let power_curve = self.repository.get_power_curve_data(session_id)
            .map_err(ExportError::from)?;

        // Create PDF
        self.create_pdf(output_path, &session.test_session, &session.motorcycle, &session.operator, session.customer.as_ref(), &test_data, &power_curve)?;

        Ok(())
    }

    fn create_pdf(
        &self,
        output_path: &str,
        session: &TestSession,
        motorcycle: &Motorcycle,
        operator: &Operator,
        customer: Option<&Customer>,
        _test_data: &[TestDataPoint],
        power_curve: &[PowerCurveData],
    ) -> Result<(), ExportError> {
        // Create PDF document
        let (doc, page1, layer1) = PdfDocument::new("DYNO TEST REPORT", Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Add fonts
        let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
        let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica)?;

        // Draw company information box
        let rect = printpdf::Rect::new(Mm(15.0), Mm(210.0), Mm(115.0), Mm(245.0));
        current_layer.add_rect(rect);
        current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
        current_layer.set_outline_thickness(1.0);

        // Company info text
        current_layer.begin_text_section();
        current_layer.set_font(&font_bold, 14.0);
        current_layer.set_text_cursor(Mm(20.0), Mm(235.0));
        current_layer.write_text("SRP", &font_bold);
        current_layer.set_font(&font_bold, 12.0);
        current_layer.set_text_cursor(Mm(20.0), Mm(230.0));
        current_layer.write_text("Sain Racing Performance", &font_bold);
        current_layer.set_font(&font_regular, 10.0);
        current_layer.set_text_cursor(Mm(20.0), Mm(225.0));
        current_layer.write_text("Jl. Dyno Test Center No. 123", &font_regular);
        current_layer.set_text_cursor(Mm(20.0), Mm(220.0));
        current_layer.write_text("tel +62 123 456789 fax +62 123 456790", &font_regular);
        current_layer.set_text_cursor(Mm(20.0), Mm(215.0));
        current_layer.write_text("www.sainracing.com -- info@sainracing.com", &font_regular);
        current_layer.end_text_section();

        // Test title (top right)
        current_layer.begin_text_section();
        current_layer.set_font(&font_bold, 24.0);
        current_layer.set_text_cursor(Mm(120.0), Mm(240.0));
        current_layer.write_text("DYNO TEST", &font_bold);
        current_layer.end_text_section();

        // Draw simple chart placeholder
        let chart_rect = printpdf::Rect::new(Mm(20.0), Mm(120.0), Mm(190.0), Mm(200.0));
        current_layer.add_rect(chart_rect);
        current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
        current_layer.set_outline_thickness(1.0);

        // Chart title
        current_layer.begin_text_section();
        current_layer.set_font(&font_bold, 12.0);
        current_layer.set_text_cursor(Mm(95.0), Mm(195.0));
        current_layer.write_text("Power & Torque Chart", &font_bold);
        current_layer.end_text_section();

        // Draw simple power curve line if data exists
        if !power_curve.is_empty() {
            self.draw_simple_chart(&current_layer, power_curve, Mm(25.0), Mm(125.0), Mm(160.0), Mm(65.0))?;
        }

        // Test information table (bottom left)
        let table_start_y = Mm(100.0);
        self.draw_test_info_table(&current_layer, &font_regular, session, motorcycle, operator, customer, Mm(20.0), table_start_y)?;

        // Test results table (bottom right)
        self.draw_results_table(&current_layer, &font_regular, session, Mm(120.0), table_start_y)?;

        // Legend for chart
        current_layer.begin_text_section();
        current_layer.set_font(&font_regular, 10.0);
        current_layer.set_text_cursor(Mm(160.0), Mm(180.0));
        current_layer.write_text("● Power (HP)", &font_regular);
        current_layer.set_text_cursor(Mm(160.0), Mm(175.0));
        current_layer.write_text("● Torque (Nm)", &font_regular);
        current_layer.end_text_section();

        // Save PDF
        doc.save(&mut BufWriter::new(File::create(output_path)?))?;

        Ok(())
    }

    fn draw_simple_chart(
        &self,
        layer: &PdfLayerReference,
        power_curve: &[PowerCurveData],
        x: Mm,
        y: Mm,
        width: Mm,
        height: Mm,
    ) -> Result<(), ExportError> {
        if power_curve.is_empty() {
            return Ok(());
        }

        // Find min/max values for scaling
        let min_rpm = power_curve.iter().map(|p| p.rpm).fold(f32::INFINITY, f32::min);
        let max_rpm = power_curve.iter().map(|p| p.rpm).fold(f32::NEG_INFINITY, f32::max);
        let max_hp = power_curve.iter().map(|p| p.horsepower).fold(f32::NEG_INFINITY, f32::max);

        // Draw simple line representation
        let mut points = Vec::new();
        for point in power_curve.iter().take(10) { // Simplify to 10 points
            let norm_x = if max_rpm > min_rpm {
                (point.rpm - min_rpm) / (max_rpm - min_rpm)
            } else {
                0.5
            };
            let norm_y = if max_hp > 0.0 {
                point.horsepower / max_hp
            } else {
                0.0
            };

            let chart_x = x.0 + (width.0 * norm_x);
            let chart_y = y.0 + (height.0 * norm_y);
            points.push(Point::new(Mm(chart_x), Mm(chart_y)));
        }

        // Draw lines between points
        for i in 0..points.len() - 1 {
            let line = printpdf::Line {
                points: vec![(points[i], false), (points[i + 1], false)],
                is_closed: false,
            };
            layer.add_line(line);
        }

        Ok(())
    }

    fn draw_test_info_table(
        &self,
        layer: &PdfLayerReference,
        font: &IndirectFontRef,
        session: &TestSession,
        motorcycle: &Motorcycle,
        operator: &Operator,
        customer: Option<&Customer>,
        x: Mm,
        y: Mm,
    ) -> Result<(), ExportError> {
        layer.begin_text_section();
        layer.set_font(font, 10.0);

        let mut current_y = y;
        let line_height = Mm(5.0);

        // Test date
        layer.set_text_cursor(x, current_y);
        layer.write_text("Test date", font);
        layer.set_text_cursor(x + Mm(30.0), current_y);
        layer.write_text(session.start_time.format("%Y-%m-%d %H:%M").to_string(), font);
        current_y -= line_height;

        // Vehicle model
        layer.set_text_cursor(x, current_y);
        layer.write_text("Vehicle model", font);
        layer.set_text_cursor(x + Mm(30.0), current_y);
        layer.write_text(format!("{} {}", motorcycle.brand, motorcycle.model), font);
        current_y -= line_height;

        // License plate
        layer.set_text_cursor(x, current_y);
        layer.write_text("Reg. plate", font);
        layer.set_text_cursor(x + Mm(30.0), current_y);
        layer.write_text(motorcycle.license_plate.as_deref().unwrap_or("N/A").to_string(), font);
        current_y -= line_height;

        // Customer
        layer.set_text_cursor(x, current_y);
        layer.write_text("Customer", font);
        layer.set_text_cursor(x + Mm(30.0), current_y);
        layer.write_text(customer.map(|c| c.name.as_str()).unwrap_or("N/A").to_string(), font);
        current_y -= line_height;

        // Operator
        layer.set_text_cursor(x, current_y);
        layer.write_text("Operator", font);
        layer.set_text_cursor(x + Mm(30.0), current_y);
        layer.write_text(operator.full_name.clone(), font);
        current_y -= line_height;

        // Engine displacement
        layer.set_text_cursor(x, current_y);
        layer.write_text("Displacement", font);
        layer.set_text_cursor(x + Mm(30.0), current_y);
        layer.write_text(format!("{} cc", motorcycle.engine_cc), font);
        current_y -= line_height;

        // Test type
        layer.set_text_cursor(x, current_y);
        layer.write_text("Test type", font);
        layer.set_text_cursor(x + Mm(30.0), current_y);
        layer.write_text(session.test_type.clone(), font);

        layer.end_text_section();
        Ok(())
    }

    fn draw_results_table(
        &self,
        layer: &PdfLayerReference,
        font: &IndirectFontRef,
        session: &TestSession,
        x: Mm,
        y: Mm,
    ) -> Result<(), ExportError> {
        layer.begin_text_section();
        layer.set_font(font, 10.0);

        let mut current_y = y;
        let line_height = Mm(5.0);

        // Max power
        layer.set_text_cursor(x, current_y);
        layer.write_text("Max power", font);
        layer.set_text_cursor(x + Mm(40.0), current_y);
        layer.write_text(format!("{:.1} HP", session.max_hp.unwrap_or(0.0)), font);
        if let Some(rpm) = session.max_hp_rpm {
            layer.set_text_cursor(x + Mm(65.0), current_y);
            layer.write_text(format!("@ {:.0} rpm", rpm), font);
        }
        current_y -= line_height;

        // Max torque
        layer.set_text_cursor(x, current_y);
        layer.write_text("Max torque", font);
        layer.set_text_cursor(x + Mm(40.0), current_y);
        layer.write_text(format!("{:.1} Nm", session.max_torque.unwrap_or(0.0)), font);
        if let Some(rpm) = session.max_torque_rpm {
            layer.set_text_cursor(x + Mm(65.0), current_y);
            layer.write_text(format!("@ {:.0} rpm", rpm), font);
        }
        current_y -= line_height * 2.0;

        // Max speed
        layer.set_text_cursor(x, current_y);
        layer.write_text("Maximum speed", font);
        layer.set_text_cursor(x + Mm(40.0), current_y);
        layer.write_text(format!("{:.0} km/h", session.max_speed.unwrap_or(0.0)), font);
        current_y -= line_height;

        // Max RPM
        layer.set_text_cursor(x, current_y);
        layer.write_text("Maximum rpm", font);
        layer.set_text_cursor(x + Mm(40.0), current_y);
        layer.write_text(format!("{:.0} rpm", session.max_rpm.unwrap_or(0.0)), font);
        current_y -= line_height * 2.0;

        // Average oil temperature
        if let Some(oil_temp) = session.avg_oil_temp {
            layer.set_text_cursor(x, current_y);
            layer.write_text("Avg oil temp", font);
            layer.set_text_cursor(x + Mm(40.0), current_y);
            layer.write_text(format!("{:.1} °C", oil_temp), font);
            current_y -= line_height;
        }

        // Average AFR
        if let Some(afr) = session.avg_afr {
            layer.set_text_cursor(x, current_y);
            layer.write_text("Avg AFR", font);
            layer.set_text_cursor(x + Mm(40.0), current_y);
            layer.write_text(format!("{:.1}", afr), font);
        }

        layer.end_text_section();
        Ok(())
    }
}