use eframe::egui;
use crate::data::db_models::{NewMotorcycle, Motorcycle};
use crate::data::repository::TestRepository;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MotorcycleForm {
    pub brand: String,
    pub model: String,
    pub year: String,
    pub engine_cc: String,
    pub vin: String,
    pub license_plate: String,
    pub notes: String,
}

impl Default for MotorcycleForm {
    fn default() -> Self {
        Self {
            brand: String::new(),
            model: String::new(),
            year: "2023".to_string(),
            engine_cc: "150".to_string(),
            vin: String::new(),
            license_plate: String::new(),
            notes: String::new(),
        }
    }
}

impl MotorcycleForm {
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        if self.brand.trim().is_empty() {
            errors.push("Brand is required".to_string());
        }
        
        if self.model.trim().is_empty() {
            errors.push("Model is required".to_string());
        }
        
        if let Err(_) = self.year.parse::<i32>() {
            errors.push("Year must be a valid number".to_string());
        } else {
            let year: i32 = self.year.parse().unwrap();
            if year < 1900 || year > 2030 {
                errors.push("Year must be between 1900 and 2030".to_string());
            }
        }
        
        if let Err(_) = self.engine_cc.parse::<i32>() {
            errors.push("Engine CC must be a valid number".to_string());
        } else {
            let cc: i32 = self.engine_cc.parse().unwrap();
            if cc < 50 || cc > 2000 {
                errors.push("Engine CC must be between 50 and 2000".to_string());
            }
        }
        
        errors
    }
    
    pub fn to_new_motorcycle(&self) -> Option<NewMotorcycle> {
        let year = self.year.parse::<i32>().ok()?;
        let engine_cc = self.engine_cc.parse::<i32>().ok()?;
        
        Some(NewMotorcycle {
            brand: self.brand.trim().to_string(),
            model: self.model.trim().to_string(),
            year,
            engine_cc,
            vin: if self.vin.trim().is_empty() { None } else { Some(self.vin.trim().to_string()) },
            license_plate: if self.license_plate.trim().is_empty() { None } else { Some(self.license_plate.trim().to_string()) },
            notes: if self.notes.trim().is_empty() { None } else { Some(self.notes.trim().to_string()) },
        })
    }
}

pub struct MotorcycleModal {
    pub show: bool,
    pub form: MotorcycleForm,
    pub errors: Vec<String>,
    pub is_loading: bool,
}

impl Default for MotorcycleModal {
    fn default() -> Self {
        Self {
            show: false,
            form: MotorcycleForm::default(),
            errors: Vec::new(),
            is_loading: false,
        }
    }
}

impl MotorcycleModal {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn open(&mut self) {
        self.show = true;
        self.form = MotorcycleForm::default();
        self.errors.clear();
        self.is_loading = false;
    }
    
    pub fn close(&mut self) {
        self.show = false;
        self.form = MotorcycleForm::default();
        self.errors.clear();
        self.is_loading = false;
    }
    
    pub fn show_modal(
        &mut self,
        ctx: &egui::Context,
        repository: Option<Arc<TestRepository>>,
    ) -> Option<Motorcycle> {
        if !self.show {
            return None;
        }
        
        let mut result = None;
        
        egui::Window::new("Motorcycle Information")
            .collapsible(false)
            .resizable(false)
            .default_width(500.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_enabled_ui(!self.is_loading, |ui| {
                
                // Header
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.heading("Enter Motorcycle Details");
                    ui.label("Fill in the motorcycle information before starting the test");
                    ui.add_space(15.0);
                });
                
                ui.separator();
                ui.add_space(10.0);
                
                // Form fields
                egui::Grid::new("motorcycle_form")
                    .num_columns(2)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        // Required fields
                        ui.label("Brand*:");
                        ui.text_edit_singleline(&mut self.form.brand);
                        ui.end_row();
                        
                        ui.label("Model*:");
                        ui.text_edit_singleline(&mut self.form.model);
                        ui.end_row();
                        
                        ui.label("Year*:");
                        ui.text_edit_singleline(&mut self.form.year);
                        ui.end_row();
                        
                        ui.label("Engine CC*:");
                        ui.text_edit_singleline(&mut self.form.engine_cc);
                        ui.end_row();
                        
                        // Optional fields
                        ui.label("VIN:");
                        ui.text_edit_singleline(&mut self.form.vin);
                        ui.end_row();
                        
                        ui.label("License Plate:");
                        ui.text_edit_singleline(&mut self.form.license_plate);
                        ui.end_row();
                        
                        ui.label("Notes:");
                        ui.text_edit_multiline(&mut self.form.notes);
                        ui.end_row();
                    });
                
                ui.add_space(10.0);
                
                // Error display
                if !self.errors.is_empty() {
                    ui.separator();
                    ui.add_space(5.0);
                    for error in &self.errors {
                        ui.colored_label(egui::Color32::RED, format!("• {}", error));
                    }
                    ui.add_space(10.0);
                }
                
                ui.separator();
                ui.add_space(10.0);
                
                // Buttons
                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width() - 200.0);
                    
                    if ui.button("Cancel").clicked() {
                        self.close();
                    }
                    
                    ui.add_space(10.0);
                    
                    let save_button = if self.is_loading {
                        ui.add_enabled(false, egui::Button::new("Saving..."))
                    } else {
                        ui.button("Save & Start Test")
                    };
                    
                    if save_button.clicked() {
                        self.errors = self.form.validate();
                        
                        if self.errors.is_empty() {
                            if let Some(new_motorcycle) = self.form.to_new_motorcycle() {
                                if let Some(repo) = repository.as_ref() {
                                    self.is_loading = true;
                                    
                                    match repo.create_motorcycle(new_motorcycle) {
                                        Ok(motorcycle) => {
                                            log::info!("Motorcycle created: {} {} {} (ID: {})", 
                                                motorcycle.brand, motorcycle.model, motorcycle.year, motorcycle.id);
                                            result = Some(motorcycle);
                                            self.close();
                                        }
                                        Err(e) => {
                                            log::error!("Failed to create motorcycle: {}", e);
                                            self.errors.push(format!("Failed to save motorcycle: {}", e));
                                            self.is_loading = false;
                                        }
                                    }
                                } else {
                                    self.errors.push("Database not available".to_string());
                                }
                            } else {
                                self.errors.push("Invalid form data".to_string());
                            }
                        }
                    }
                });
                });
                
                ui.add_space(10.0);
                ui.small("* Required fields");
            });
        
        result
    }
}

// Helper component for displaying motorcycle info
pub struct MotorcycleInfoPanel {
    current_motorcycle: Option<Motorcycle>,
}

impl Default for MotorcycleInfoPanel {
    fn default() -> Self {
        Self {
            current_motorcycle: None,
        }
    }
}

impl MotorcycleInfoPanel {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set_motorcycle(&mut self, motorcycle: Option<Motorcycle>) {
        self.current_motorcycle = motorcycle;
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Current Motorcycle");
                ui.separator();
                ui.add_space(5.0);
                
                if let Some(motorcycle) = &self.current_motorcycle {
                    egui::Grid::new("motorcycle_info")
                        .num_columns(2)
                        .spacing([10.0, 5.0])
                        .show(ui, |ui| {
                            ui.label("Brand:");
                            ui.strong(&motorcycle.brand);
                            ui.end_row();
                            
                            ui.label("Model:");
                            ui.strong(&motorcycle.model);
                            ui.end_row();
                            
                            ui.label("Year:");
                            ui.strong(&motorcycle.year.to_string());
                            ui.end_row();
                            
                            ui.label("Engine:");
                            ui.strong(&format!("{}cc", motorcycle.engine_cc));
                            ui.end_row();
                            
                            if let Some(vin) = &motorcycle.vin {
                                ui.label("VIN:");
                                ui.label(vin);
                                ui.end_row();
                            }
                            
                            if let Some(plate) = &motorcycle.license_plate {
                                ui.label("License:");
                                ui.label(plate);
                                ui.end_row();
                            }
                            
                            if let Some(notes) = &motorcycle.notes {
                                if !notes.is_empty() {
                                    ui.label("Notes:");
                                    ui.label(notes);
                                    ui.end_row();
                                }
                            }
                        });
                    
                    ui.add_space(10.0);
                    
                    // Status indicators
                    ui.horizontal(|ui| {
                        ui.label("Status:");
                        ui.colored_label(egui::Color32::GREEN, "● Ready for testing");
                    });
                    
                    ui.add_space(5.0);
                    
                    if ui.button("Change Motorcycle").clicked() {
                        self.current_motorcycle = None;
                    }
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label("No motorcycle selected");
                        ui.add_space(10.0);
                        ui.small("Select a motorcycle to start testing");
                        ui.add_space(20.0);
                    });
                }
            });
        });
    }
}