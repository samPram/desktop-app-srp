use egui::{Ui, RichText};

use crate::app::{AppState, Menu};

pub fn sidebar_ui(ui: &mut Ui, state: &mut AppState) {
    ui.vertical_centered(|ui| {
        ui.add_space(6.0);
        ui.heading("Menu");
    });
    ui.separator();

    let items = [
        (Menu::Dashboard, "Dashboard"),
        (Menu::DynData, "Dyn Data"),
        (Menu::Cepols, "Cepols"),
        (Menu::RunHistory, "Run History"),
        (Menu::Runs, "Runs"),
        (Menu::Reports, "Reports"),
        (Menu::Configuration, "Configuration"),
    ];

    for (menu, label) in items.iter() {
        let is_active = state.active_menu == *menu;
        let text = if is_active {
            RichText::new(*label).strong().underline()
        } else {
            RichText::new(*label)
        };

        if ui.button(text).clicked() {
            state.active_menu = *menu;
        }
    }
}
