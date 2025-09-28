use eframe::egui::{self, Color32, RichText, Stroke};
use egui_extras::{Column, TableBuilder};

#[derive(Debug, Clone)]
pub struct TestRun {
    pub id: u32,
    pub date: String,
    pub time: String,
    pub motorcycle: String,
    pub operator: String,
    pub max_hp: f32,
    pub max_torque: f32,
    pub max_rpm: f32,
    pub test_duration: String,
    pub status: String,
    pub notes: String,
}

pub struct RunHistory {
    test_runs: Vec<TestRun>,
    selected_run: Option<usize>,
    sort_column: SortColumn,
    sort_ascending: bool,
    search_text: String,
}

#[derive(Debug, Clone, PartialEq)]
enum SortColumn {
    Id,
    Date,
    Motorcycle,
    Operator,
    MaxHp,
    MaxTorque,
    MaxRpm,
    Status,
}

impl RunHistory {
    pub fn new() -> Self {
        Self {
            test_runs: Self::generate_sample_data(),
            selected_run: None,
            sort_column: SortColumn::Id,
            sort_ascending: false, // Default to newest first
            search_text: String::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        // Set dark theme
        let mut style = (*ui.ctx().style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.override_text_color = Some(Color32::WHITE);
        style.visuals.panel_fill = Color32::from_rgb(30, 30, 30);
        style.visuals.window_fill = Color32::from_rgb(25, 25, 25);
        ui.ctx().set_style(style);

        ui.vertical(|ui| {
            // Header section
            self.show_header(ui);
            
            ui.add_space(15.0);
            
            // Search and filter controls
            self.show_search_controls(ui);
            
            ui.add_space(15.0);

            // Main table
            self.show_table(ui);
            
            ui.add_space(15.0);
            
            // Details panel for selected run
            if let Some(selected_idx) = self.selected_run {
                if selected_idx < self.test_runs.len() {
                    self.show_run_details(ui, &self.test_runs[selected_idx].clone());
                }
            }
        });
    }

    fn show_header(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("🏍️ Run History")
                    .size(24.0)
                    .strong()
                    .color(Color32::WHITE)
            );
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Summary stats
                let total_runs = self.test_runs.len();
                let completed_runs = self.test_runs.iter().filter(|r| r.status == "Completed").count();
                
                ui.label(
                    RichText::new(&format!("Total: {} | Completed: {}", total_runs, completed_runs))
                        .size(12.0)
                        .color(Color32::LIGHT_GRAY)
                );
            });
        });
        
        ui.separator();
    }

    fn show_search_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("🔍 Search:");
            ui.text_edit_singleline(&mut self.search_text);
            
            ui.add_space(20.0);
            
            if ui.button("📤 Export").clicked() {
                // TODO: Implement export functionality
            }
            
            if ui.button("🗑 Clear History").clicked() {
                // TODO: Implement clear history with confirmation
            }
            
            if ui.button("🔄 Refresh").clicked() {
                // TODO: Implement refresh from database
            }
        });
    }

    fn show_table(&mut self, ui: &mut egui::Ui) {
        let mut clicked_row: Option<usize> = None;
        let mut sort_request: Option<SortColumn> = None;

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().at_least(50.0))  // ID
            .column(Column::auto().at_least(100.0)) // Date
            .column(Column::auto().at_least(80.0))  // Time
            .column(Column::auto().at_least(120.0)) // Motorcycle
            .column(Column::auto().at_least(100.0)) // Operator
            .column(Column::auto().at_least(80.0))  // Max HP
            .column(Column::auto().at_least(80.0))  // Max Torque
            .column(Column::auto().at_least(80.0))  // Max RPM
            .column(Column::auto().at_least(80.0))  // Duration
            .column(Column::auto().at_least(80.0))  // Status
            .column(Column::remainder().at_least(100.0)) // Notes
            .header(25.0, |mut header| {
                header.col(|ui| {
                    if ui.button(RichText::new("ID").strong()).clicked() {
                        sort_request = Some(SortColumn::Id);
                    }
                });
                header.col(|ui| {
                    if ui.button(RichText::new("Date").strong()).clicked() {
                        sort_request = Some(SortColumn::Date);
                    }
                });
                header.col(|ui| {
                    ui.label(RichText::new("Time").strong());
                });
                header.col(|ui| {
                    if ui.button(RichText::new("Motorcycle").strong()).clicked() {
                        sort_request = Some(SortColumn::Motorcycle);
                    }
                });
                header.col(|ui| {
                    if ui.button(RichText::new("Operator").strong()).clicked() {
                        sort_request = Some(SortColumn::Operator);
                    }
                });
                header.col(|ui| {
                    if ui.button(RichText::new("Max HP").strong()).clicked() {
                        sort_request = Some(SortColumn::MaxHp);
                    }
                });
                header.col(|ui| {
                    if ui.button(RichText::new("Max Torque").strong()).clicked() {
                        sort_request = Some(SortColumn::MaxTorque);
                    }
                });
                header.col(|ui| {
                    if ui.button(RichText::new("Max RPM").strong()).clicked() {
                        sort_request = Some(SortColumn::MaxRpm);
                    }
                });
                header.col(|ui| {
                    ui.label(RichText::new("Duration").strong());
                });
                header.col(|ui| {
                    if ui.button(RichText::new("Status").strong()).clicked() {
                        sort_request = Some(SortColumn::Status);
                    }
                });
                header.col(|ui| {
                    ui.label(RichText::new("Notes").strong());
                });
            })
            .body(|mut body| {
                let filtered_runs = self.get_filtered_runs();
                
                for (idx, run) in filtered_runs.iter().enumerate() {
                    let row_height = 25.0;
                    let is_selected = self.selected_run == Some(idx);
                    
                    body.row(row_height, |mut row| {
                        // ID column
                        row.col(|ui| {
                            let response = ui.selectable_label(is_selected, format!("#{:03}", run.id));
                            if response.clicked() {
                                clicked_row = Some(idx);
                            }
                        });
                        
                        // Date column
                        row.col(|ui| {
                            let response = ui.selectable_label(is_selected, &run.date);
                            if response.clicked() {
                                clicked_row = Some(idx);
                            }
                        });
                        
                        // Time column
                        row.col(|ui| {
                            let response = ui.selectable_label(is_selected, &run.time);
                            if response.clicked() {
                                clicked_row = Some(idx);
                            }
                        });
                        
                        // Motorcycle column
                        row.col(|ui| {
                            let response = ui.selectable_label(is_selected, &run.motorcycle);
                            if response.clicked() {
                                clicked_row = Some(idx);
                            }
                        });
                        
                        // Operator column
                        row.col(|ui| {
                            let response = ui.selectable_label(is_selected, &run.operator);
                            if response.clicked() {
                                clicked_row = Some(idx);
                            }
                        });
                        
                        // Max HP column with color coding
                        row.col(|ui| {
                            let color = if run.max_hp > 150.0 {
                                Color32::from_rgb(100, 255, 100) // Green for high power
                            } else if run.max_hp > 100.0 {
                                Color32::from_rgb(255, 255, 100) // Yellow for medium power
                            } else {
                                Color32::WHITE
                            };
                            let response = ui.selectable_label(
                                is_selected, 
                                RichText::new(format!("{:.1}", run.max_hp)).color(color)
                            );
                            if response.clicked() {
                                clicked_row = Some(idx);
                            }
                        });
                        
                        // Max Torque column with color coding
                        row.col(|ui| {
                            let color = if run.max_torque > 120.0 {
                                Color32::from_rgb(100, 255, 100) // Green for high torque
                            } else if run.max_torque > 80.0 {
                                Color32::from_rgb(255, 255, 100) // Yellow for medium torque
                            } else {
                                Color32::WHITE
                            };
                            let response = ui.selectable_label(
                                is_selected, 
                                RichText::new(format!("{:.1}", run.max_torque)).color(color)
                            );
                            if response.clicked() {
                                clicked_row = Some(idx);
                            }
                        });
                        
                        // Max RPM column
                        row.col(|ui| {
                            let response = ui.selectable_label(is_selected, format!("{:.0}", run.max_rpm));
                            if response.clicked() {
                                clicked_row = Some(idx);
                            }
                        });
                        
                        // Duration column
                        row.col(|ui| {
                            let response = ui.selectable_label(is_selected, &run.test_duration);
                            if response.clicked() {
                                clicked_row = Some(idx);
                            }
                        });
                        
                        // Status column with color coding
                        row.col(|ui| {
                            let (color, icon) = match run.status.as_str() {
                                "Completed" => (Color32::from_rgb(100, 255, 100), "✅"),
                                "Failed" => (Color32::from_rgb(255, 100, 100), "❌"),
                                "Interrupted" => (Color32::from_rgb(255, 200, 100), "⚠️"),
                                _ => (Color32::WHITE, "❓"),
                            };
                            let response = ui.selectable_label(
                                is_selected, 
                                RichText::new(format!("{} {}", icon, run.status)).color(color)
                            );
                            if response.clicked() {
                                clicked_row = Some(idx);
                            }
                        });
                        
                        // Notes column (truncated)
                        row.col(|ui| {
                            let truncated = if run.notes.len() > 30 {
                                format!("{}...", &run.notes[..27])
                            } else {
                                run.notes.clone()
                            };
                            let response = ui.selectable_label(is_selected, truncated);
                            if response.clicked() {
                                clicked_row = Some(idx);
                            }
                        });
                    });
                }
            });

        // Handle sort request after table is drawn
        if let Some(column) = sort_request {
            self.sort_by(column);
        }

        // Handle row selection after table is drawn
        if let Some(idx) = clicked_row {
            self.selected_run = if self.selected_run == Some(idx) { None } else { Some(idx) };
        }
    }

    fn show_run_details(&self, ui: &mut egui::Ui, run: &TestRun) {
        ui.separator();
        ui.label(RichText::new("📋 Run Details").size(18.0).strong());
        
        egui::Frame::none()
            .fill(Color32::from_rgb(35, 35, 35))
            .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 60)))
            .rounding(8.0)
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.columns(3, |columns| {
                    // Left column - Basic info
                    columns[0].vertical(|ui| {
                        ui.label(RichText::new("Basic Information").strong().color(Color32::LIGHT_BLUE));
                        ui.add_space(10.0);
                        
                        ui.horizontal(|ui| {
                            ui.label("Run ID:");
                            ui.label(RichText::new(format!("#{:03}", run.id)).strong());
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Date & Time:");
                            ui.label(format!("{} {}", run.date, run.time));
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Motorcycle:");
                            ui.label(RichText::new(&run.motorcycle).strong());
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Operator:");
                            ui.label(&run.operator);
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Duration:");
                            ui.label(&run.test_duration);
                        });
                    });
                    
                    // Middle column - Performance data
                    columns[1].vertical(|ui| {
                        ui.label(RichText::new("Performance Data").strong().color(Color32::LIGHT_GREEN));
                        ui.add_space(10.0);
                        
                        ui.horizontal(|ui| {
                            ui.label("Max Power:");
                            ui.label(RichText::new(format!("{:.1} HP", run.max_hp)).strong().color(Color32::from_rgb(255, 100, 100)));
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Max Torque:");
                            ui.label(RichText::new(format!("{:.1} Nm", run.max_torque)).strong().color(Color32::from_rgb(100, 150, 255)));
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Max RPM:");
                            ui.label(format!("{:.0} RPM", run.max_rpm));
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Power/Weight:");
                            ui.label(format!("{:.2} HP/kg", run.max_hp / 180.0)); // Assuming 180kg bike+rider
                        });
                    });
                    
                    // Right column - Status and actions
                    columns[2].vertical(|ui| {
                        ui.label(RichText::new("Status & Actions").strong().color(Color32::YELLOW));
                        ui.add_space(10.0);
                        
                        let (status_color, status_icon) = match run.status.as_str() {
                            "Completed" => (Color32::from_rgb(100, 255, 100), "✅"),
                            "Failed" => (Color32::from_rgb(255, 100, 100), "❌"),
                            "Interrupted" => (Color32::from_rgb(255, 200, 100), "⚠️"),
                            _ => (Color32::WHITE, "❓"),
                        };
                        
                        ui.horizontal(|ui| {
                            ui.label("Status:");
                            ui.label(RichText::new(format!("{} {}", status_icon, run.status)).color(status_color));
                        });
                        
                        ui.add_space(10.0);
                        
                        if ui.button("📊 View Chart").clicked() {
                            // TODO: Open detailed chart view
                        }
                        
                        if ui.button("📄 Export Report").clicked() {
                            // TODO: Export run report
                        }
                        
                        if ui.button("🗑️ Delete Run").clicked() {
                            // TODO: Delete run with confirmation
                        }
                    });
                });
                
                // Notes section at bottom
                if !run.notes.is_empty() {
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.label(RichText::new("Notes:").strong());
                    ui.label(&run.notes);
                }
            });
    }

    fn sort_by(&mut self, column: SortColumn) {
        if self.sort_column == column {
            self.sort_ascending = !self.sort_ascending;
        } else {
            self.sort_column = column;
            self.sort_ascending = true;
        }
        
        self.test_runs.sort_by(|a, b| {
            let ordering = match self.sort_column {
                SortColumn::Id => a.id.cmp(&b.id),
                SortColumn::Date => a.date.cmp(&b.date),
                SortColumn::Motorcycle => a.motorcycle.cmp(&b.motorcycle),
                SortColumn::Operator => a.operator.cmp(&b.operator),
                SortColumn::MaxHp => a.max_hp.partial_cmp(&b.max_hp).unwrap_or(std::cmp::Ordering::Equal),
                SortColumn::MaxTorque => a.max_torque.partial_cmp(&b.max_torque).unwrap_or(std::cmp::Ordering::Equal),
                SortColumn::MaxRpm => a.max_rpm.partial_cmp(&b.max_rpm).unwrap_or(std::cmp::Ordering::Equal),
                SortColumn::Status => a.status.cmp(&b.status),
            };
            
            if self.sort_ascending {
                ordering
            } else {
                ordering.reverse()
            }
        });
    }

    fn get_filtered_runs(&self) -> Vec<&TestRun> {
        if self.search_text.is_empty() {
            self.test_runs.iter().collect()
        } else {
            let search_lower = self.search_text.to_lowercase();
            self.test_runs
                .iter()
                .filter(|run| {
                    run.motorcycle.to_lowercase().contains(&search_lower)
                        || run.operator.to_lowercase().contains(&search_lower)
                        || run.notes.to_lowercase().contains(&search_lower)
                        || run.status.to_lowercase().contains(&search_lower)
                })
                .collect()
        }
    }

    fn generate_sample_data() -> Vec<TestRun> {
        vec![
            TestRun {
                id: 1,
                date: "2024-12-01".to_string(),
                time: "09:15:30".to_string(),
                motorcycle: "Yamaha YZF-R1".to_string(),
                operator: "John Smith".to_string(),
                max_hp: 182.5,
                max_torque: 115.8,
                max_rpm: 11500.0,
                test_duration: "45s".to_string(),
                status: "Completed".to_string(),
                notes: "Excellent run, bike running perfectly. New exhaust system installed.".to_string(),
            },
            TestRun {
                id: 2,
                date: "2024-12-01".to_string(),
                time: "10:30:45".to_string(),
                motorcycle: "Honda CBR1000RR".to_string(),
                operator: "Jane Doe".to_string(),
                max_hp: 175.2,
                max_torque: 108.3,
                max_rpm: 12200.0,
                test_duration: "42s".to_string(),
                status: "Completed".to_string(),
                notes: "Good baseline run before modifications.".to_string(),
            },
            TestRun {
                id: 3,
                date: "2024-11-30".to_string(),
                time: "14:20:15".to_string(),
                motorcycle: "Kawasaki ZX-10R".to_string(),
                operator: "Mike Johnson".to_string(),
                max_hp: 195.8,
                max_torque: 125.4,
                max_rpm: 11800.0,
                test_duration: "48s".to_string(),
                status: "Completed".to_string(),
                notes: "Impressive power figures with aftermarket tune.".to_string(),
            },
            TestRun {
                id: 4,
                date: "2024-11-30".to_string(),
                time: "15:45:20".to_string(),
                motorcycle: "Suzuki GSX-R1000".to_string(),
                operator: "Sarah Wilson".to_string(),
                max_hp: 0.0,
                max_torque: 0.0,
                max_rpm: 0.0,
                test_duration: "12s".to_string(),
                status: "Failed".to_string(),
                notes: "Test interrupted due to mechanical issue - oil leak detected.".to_string(),
            },
            TestRun {
                id: 5,
                date: "2024-11-29".to_string(),
                time: "11:10:00".to_string(),
                motorcycle: "Ducati Panigale V4".to_string(),
                operator: "Carlos Rodriguez".to_string(),
                max_hp: 201.3,
                max_torque: 118.7,
                max_rpm: 12500.0,
                test_duration: "50s".to_string(),
                status: "Completed".to_string(),
                notes: "Outstanding performance from the Italian machine.".to_string(),
            },
            TestRun {
                id: 6,
                date: "2024-11-29".to_string(),
                time: "16:30:10".to_string(),
                motorcycle: "BMW S1000RR".to_string(),
                operator: "Emma Thompson".to_string(),
                max_hp: 188.9,
                max_torque: 112.5,
                max_rpm: 11900.0,
                test_duration: "30s".to_string(),
                status: "Interrupted".to_string(),
                notes: "Test stopped early due to rain - partial results only.".to_string(),
            },
            TestRun {
                id: 7,
                date: "2024-11-28".to_string(),
                time: "13:45:30".to_string(),
                motorcycle: "Aprilia RSV4".to_string(),
                operator: "David Kim".to_string(),
                max_hp: 178.4,
                max_torque: 107.2,
                max_rpm: 12000.0,
                test_duration: "44s".to_string(),
                status: "Completed".to_string(),
                notes: "Solid performance, consistent with manufacturer specs.".to_string(),
            },
            TestRun {
                id: 8,
                date: "2024-11-28".to_string(),
                time: "09:20:45".to_string(),
                motorcycle: "KTM 1290 Super Duke R".to_string(),
                operator: "Lisa Garcia".to_string(),
                max_hp: 165.7,
                max_torque: 140.2,
                max_rpm: 9800.0,
                test_duration: "38s".to_string(),
                status: "Completed".to_string(),
                notes: "Excellent torque curve for a naked bike. Great low-end power.".to_string(),
            },
        ]
    }
}