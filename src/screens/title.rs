//! The title screen that appears when the game starts.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, show_title_screen.run_if(in_state(Screen::Title)));
}

// Our `egui` title screen function to replace `spawn_title_screen`
fn show_title_screen(
    mut egui_context: EguiContexts,
    mut next_screen: ResMut<NextState<Screen>>,
    mut app_exit: EventWriter<AppExit>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("My Game");

            // Play Button
            if ui.button("Play").clicked() {
                next_screen.set(Screen::Gameplay);
            }

            // High Score Button
            if ui.button("HighScore").clicked() {
                next_screen.set(Screen::HighScore);
            }

            // Exit Button, not available for Web
            #[cfg(not(target_family = "wasm"))]
            if ui.button("Exit").clicked() {
                app_exit.send(AppExit::Success);
            }
        });
    });
}
