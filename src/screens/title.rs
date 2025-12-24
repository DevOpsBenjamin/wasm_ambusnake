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
    #[cfg_attr(target_family = "wasm", allow(unused))] mut app_exit: EventWriter<AppExit>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        // Adding padding around the panel
        ui.add_space(50.0);

        ui.vertical_centered(|ui| {
            // Title with larger font size
            ui.add_space(80.0); // Adjust space above the title if needed
            ui.label(
                egui::RichText::new("AmbuSnake")
                    .size(80.0) // Larger font size for title
                    .strong(),
            );

            // Additional space between the title and buttons
            ui.add_space(70.0);

            // Adjusted button group alignment with spacing in between
            if ui
                .add_sized(
                    [220.0, 80.0],
                    egui::Button::new(
                        egui::RichText::new("Play").size(35.0), // Adjusted font size
                    ),
                )
                .clicked()
            {
                next_screen.set(Screen::Gameplay);
            }

            ui.add_space(15.0); // Spacing between buttons

            if ui
                .add_sized(
                    [220.0, 80.0],
                    egui::Button::new(egui::RichText::new("HighScore").size(35.0)),
                )
                .clicked()
            {
                next_screen.set(Screen::HighScore);
            }

            #[cfg(not(target_family = "wasm"))]
            {
                ui.add_space(15.0); // Spacing between buttons
                if ui
                    .add_sized(
                        [220.0, 80.0],
                        egui::Button::new(egui::RichText::new("Exit").size(35.0)),
                    )
                    .clicked()
                {
                    app_exit.send(AppExit::Success);
                }
            }

            // Adding padding below the button group
            ui.add_space(50.0);
        });
    });
}
