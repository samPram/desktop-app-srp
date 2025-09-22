use eframe::egui::{self, Color32, RichText, Stroke};

#[derive(Debug, Clone, PartialEq)]
pub enum SidebarItem {
    Dashboard,
    DynData,
    Reports,
    Runs,
    RunHistory,
    Configuration,
}

pub struct Sidebar {
    selected_item: SidebarItem,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            selected_item: SidebarItem::Dashboard,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.set_min_width(200.0);
            ui.set_max_width(200.0);
            
            // Logo/Title area
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(RichText::new("MotoDyno").size(18.0).strong());
                ui.label(RichText::new("v1.0.0").size(12.0).weak());
                ui.add_space(15.0);
            });

            ui.separator();
            ui.add_space(10.0);

            // Navigation items
            self.nav_item(ui, "• Dashboard", SidebarItem::Dashboard);
            self.nav_item(ui, "• Dyn Data", SidebarItem::DynData);
            
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            
            self.nav_item(ui, "• Reports", SidebarItem::Reports);
            self.nav_item(ui, "• Runs", SidebarItem::Runs);
            self.nav_item(ui, "• Run History", SidebarItem::RunHistory);
            
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            
            self.nav_item(ui, "• Configuration", SidebarItem::Configuration);
        });
    }

    fn nav_item(&mut self, ui: &mut egui::Ui, text: &str, item: SidebarItem) {
        let is_selected = self.selected_item == item;
        
        let button_color = if is_selected {
            Color32::from_rgb(70, 130, 180)
        } else {
            Color32::TRANSPARENT
        };

        let text_color = if is_selected {
            Color32::WHITE
        } else {
            Color32::LIGHT_GRAY
        };

        let response = ui.add_sized(
            [180.0, 35.0],
            egui::Button::new(RichText::new(text).color(text_color).size(14.0))
                .fill(button_color)
                .stroke(Stroke::NONE)
        );

        if response.clicked() {
            self.selected_item = item;
        }
    }

    pub fn selected_item(&self) -> &SidebarItem {
        &self.selected_item
    }
}