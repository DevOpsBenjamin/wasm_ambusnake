use bevy::prelude::*;
use bevy_egui::{
    egui::{self, TextEdit},
    EguiContexts,
};

use crate::{game::score::ScoreManager, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(PlayerName::default());
    app.add_systems(Update, show_death_screen.run_if(in_state(Screen::Death)));
}

// Define a resource to hold the player's name
#[derive(Default, Resource)]
struct PlayerName {
    name: String,
}

fn show_death_screen(
    mut egui_context: EguiContexts,
    mut next_screen: ResMut<NextState<Screen>>,
    mut player_name: ResMut<PlayerName>,
    query_score: Query<&ScoreManager>,
) {
    // Return early if there is no single `ScoreManager` component
    let score_manager = match query_score.get_single() {
        Ok(score_manager) => score_manager,
        Err(_) => return,
    };

    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        // Adding padding around the panel
        ui.add_space(50.0);

        ui.vertical_centered(|ui| {
            // Title with larger font size
            ui.add_space(40.0); // Adjust space above the title if needed
            ui.label(
                egui::RichText::new("Who will get this patient now?")
                    .size(40.0) // Larger font size for title
                    .strong(),
            );

            // Display the score
            ui.add_space(30.0); // Adjust space above the title if needed
            ui.label(
                egui::RichText::new(format!("Score: {}", score_manager.current_score)).size(30.0),
            );

            // Prompt for user input
            ui.add_space(20.0); // Adjust space above the title if needed
            ui.label(
                egui::RichText::new("Type your name and hit enter to save highscore").size(30.0),
            );

            // Input field for the user's name with specified size and custom font size
            let input = TextEdit::singleline(&mut player_name.name).hint_text("Enter your name"); // Use a larger font

            // Set the size of the input field
            ui.add_sized(
                [300.0, 40.0], // Specify width and height for the input field
                input,
            );
            // Check if the Enter key was pressed
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                // Logic to save the high score with the player's name
                save_highscore(&player_name.name, score_manager.current_score);
                next_screen.set(Screen::HighScore);
            }

            ui.add_space(50.0); // Adjust space above the button if needed
            if ui
                .add_sized(
                    [220.0, 80.0],
                    egui::Button::new(egui::RichText::new("Back").size(35.0)),
                )
                .clicked()
            {
                next_screen.set(Screen::Title);
            }
        });
    });
}

fn save_highscore(name: &str, score: usize) {
    // Implement the logic to save the high score using the player's name
    println!("High score [{}] for player: {}", score, name);
}
