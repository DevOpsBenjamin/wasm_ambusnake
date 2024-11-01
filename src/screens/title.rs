//! The title screen that appears when the game starts.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, show_title_screen.run_if(in_state(Screen::Title)));
}


fn show_title_screen(
    mut egui_context: EguiContexts,
    mut next_screen: ResMut<NextState<Screen>>,
    mut app_exit: EventWriter<AppExit>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            // Set larger font for the title
            ui.add_space(100.0); // Adjust space above if needed
            ui.label(
                egui::RichText::new("My Game")
                    .size(50.0) // Larger font size
                    .strong(),
            );

            ui.add_space(50.0); // Space between title and buttons

            // Larger buttons with more spacing
            if ui.add_sized([200.0, 60.0], egui::Button::new("Play")).clicked() {
                next_screen.set(Screen::Gameplay);
            }

            if ui.add_sized([200.0, 60.0], egui::Button::new("HighScore")).clicked() {
                next_screen.set(Screen::HighScore);
            }

            #[cfg(not(target_family = "wasm"))]
            if ui.add_sized([200.0, 60.0], egui::Button::new("Exit")).clicked() {
                app_exit.send(AppExit::Success);
            }
        });
    });
}
