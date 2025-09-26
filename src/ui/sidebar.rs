use eframe::egui::{self, Color32, Frame, Margin, RichText, Rounding, Stroke, Vec2};

#[derive(Debug, Clone, PartialEq)]
pub enum SidebarItem {
    Dashboard,
    DynData,
    RunHistory,
    Reports,
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

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<SidebarItem> {
        let mut selected_item = None;

        // Dark sidebar background
        let sidebar_frame = Frame::none()
            .fill(Color32::from_rgb(45, 45, 50))
            .inner_margin(Margin::same(0.0));

        sidebar_frame.show(ui, |ui| {
            ui.set_min_width(220.0);
            ui.set_max_width(220.0);
            ui.set_min_height(ui.available_height());

            ui.vertical(|ui| {
                // Logo/Title area with better styling
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);

                    // App title with gradient-like effect
                    ui.label(
                        RichText::new("SRP Dyno Test")
                            .size(22.0)
                            .strong()
                            .color(Color32::from_rgb(255, 255, 255)),
                    );
                    ui.label(
                        RichText::new("v1.0.0")
                            .size(11.0)
                            .color(Color32::from_rgb(160, 160, 160)),
                    );
                    ui.add_space(25.0);
                });

                // Main navigation section
                ui.add_space(5.0);

                if self.nav_item_with_icon(ui, "⚡", "Dashboard", SidebarItem::Dashboard) {
                    selected_item = Some(SidebarItem::Dashboard);
                }

                if self.nav_item_with_icon(ui, "📊", "Dyn Data", SidebarItem::DynData) {
                    selected_item = Some(SidebarItem::DynData);
                }

                // if self.nav_item_with_icon(ui, "🎛️", "Cepols", SidebarItem::Cepols) {
                //     selected_item = Some(SidebarItem::Cepols);
                // }

                // Second section separator
                // ui.add_space(15.0);
                // ui.horizontal(|ui| {
                //     ui.add_space(20.0);
                //     ui.separator();
                // });
                // ui.add_space(10.0);

                // if self.nav_item_with_icon(ui, "🏃", "Runs", SidebarItem::Runs) {
                //     selected_item = Some(SidebarItem::Runs);
                // }

                if self.nav_item_with_icon(ui, "📝", "Run History", SidebarItem::RunHistory) {
                    selected_item = Some(SidebarItem::RunHistory);
                }

                // Third section separator
                // ui.add_space(15.0);
                // ui.horizontal(|ui| {
                //     ui.add_space(20.0);
                //     ui.separator();
                // });
                // ui.add_space(10.0);

                if self.nav_item_with_icon(ui, "📋", "Reports", SidebarItem::Reports) {
                    selected_item = Some(SidebarItem::Reports);
                }

                // if self.nav_item_with_icon(ui, "📈", "Seports", SidebarItem::Seports) {
                //     selected_item = Some(SidebarItem::Seports);
                // }

                // Configuration at bottom
                // ui.add_space(15.0);
                // ui.horizontal(|ui| {
                //     ui.add_space(20.0);
                //     ui.separator();
                // });
                // ui.add_space(10.0);

                if self.nav_item_with_icon(ui, "🔧", "Configuration", SidebarItem::Configuration)
                {
                    selected_item = Some(SidebarItem::Configuration);
                }

                // Push remaining space to bottom
                ui.allocate_response(Vec2::new(1.0, ui.available_height()), egui::Sense::hover());
            });
        });

        selected_item
    }

    fn nav_item_with_icon(
        &mut self,
        ui: &mut egui::Ui,
        icon: &str,
        text: &str,
        item: SidebarItem,
    ) -> bool {
        let is_selected = self.selected_item == item;
        let mut clicked = false;

        // Colors for different states
        let (bg_color, text_color, hover_color) = if is_selected {
            (
                Color32::from_rgb(60, 60, 70),
                Color32::WHITE,
                Color32::from_rgb(70, 70, 80),
            )
        } else {
            (
                Color32::TRANSPARENT,
                Color32::from_rgb(200, 200, 200),
                Color32::from_rgb(50, 50, 55),
            )
        };

        ui.horizontal(|ui| {
            ui.add_space(15.0);

            let button_response =
                ui.allocate_response(Vec2::new(190.0, 40.0), egui::Sense::click());

            // Draw background with hover effect
            let bg_fill = if button_response.hovered() && !is_selected {
                hover_color
            } else {
                bg_color
            };

            if bg_fill != Color32::TRANSPARENT {
                ui.painter()
                    .rect_filled(button_response.rect, Rounding::same(6.0), bg_fill);
            }

            // Draw selection indicator (left border)
            if is_selected {
                let indicator_rect = egui::Rect::from_min_size(
                    button_response.rect.left_top(),
                    Vec2::new(3.0, button_response.rect.height()),
                );
                ui.painter().rect_filled(
                    indicator_rect,
                    Rounding::same(1.5),
                    Color32::from_rgb(70, 130, 255),
                );
            }

            // Position icon and text
            let icon_pos = button_response.rect.left_center() + Vec2::new(15.0, 0.0);
            let text_pos = button_response.rect.left_center() + Vec2::new(45.0, 0.0);

            // Draw icon
            ui.painter().text(
                icon_pos,
                egui::Align2::LEFT_CENTER,
                icon,
                egui::FontId::proportional(16.0),
                text_color,
            );

            // Draw text
            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                text,
                egui::FontId::proportional(14.0),
                text_color,
            );

            if button_response.clicked() {
                self.selected_item = item.clone();
                clicked = true;
            }
        });

        ui.add_space(2.0);
        clicked
    }

    // Alternative method without icons for simpler look
    fn nav_item_simple(&mut self, ui: &mut egui::Ui, text: &str, item: SidebarItem) -> bool {
        let is_selected = self.selected_item == item;
        let mut clicked = false;

        ui.horizontal(|ui| {
            ui.add_space(10.0);

            let button_color = if is_selected {
                Color32::from_rgb(60, 60, 70)
            } else {
                Color32::TRANSPARENT
            };

            let text_color = if is_selected {
                Color32::WHITE
            } else {
                Color32::from_rgb(200, 200, 200)
            };

            let response = ui.add_sized(
                [200.0, 35.0],
                egui::Button::new(RichText::new(text).color(text_color).size(14.0))
                    .fill(button_color)
                    .stroke(Stroke::NONE)
                    .rounding(Rounding::same(6.0)),
            );

            if response.clicked() {
                self.selected_item = item;
                clicked = true;
            }
        });

        ui.add_space(3.0);
        clicked
    }

    pub fn selected_item(&self) -> &SidebarItem {
        &self.selected_item
    }
}
