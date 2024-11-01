//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        show_highscore_screen.run_if(in_state(Screen::HighScore)),
    );
}

fn show_highscore_screen(
    mut egui_context: EguiContexts,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        // Adding padding around the panel
        ui.add_space(50.0);

        ui.vertical_centered(|ui| {
            // Highscore title with larger font size
            ui.add_space(80.0); // Adjust space above the title if needed
            ui.label(
                egui::RichText::new("HIGHSCORE")
                    .size(60.0) // Larger font size for highscore title
                    .strong(),
            );

            // Additional space between the title and score entries
            ui.add_space(30.0);

            // Example highscore entry
            ui.label(egui::RichText::new("Jebik - 42").size(30.0));

            // Space between the scores and back button
            ui.add_space(50.0);

            // Back button centered and styled similarly to other screens
            if ui
                .add_sized(
                    [220.0, 80.0],
                    egui::Button::new(egui::RichText::new("Back").size(35.0)),
                )
                .clicked()
            {
                next_screen.set(Screen::Title);
            }

            // Adding padding below the button group
            ui.add_space(50.0);
        });
    });
}
