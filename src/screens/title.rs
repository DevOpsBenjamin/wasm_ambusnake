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
                    egui::Button::new(
                        egui::RichText::new("HighScore").size(35.0),
                    ),
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
                        egui::Button::new(
                            egui::RichText::new("Exit").size(35.0),
                        ),
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

            /*
fn show_title_screen(
    mut egui_context: EguiContexts,
    mut next_screen: ResMut<NextState<Screen>>,
    mut app_exit: EventWriter<AppExit>,
    snake_assets: Res<SnakeAssets>,
    bonus_assets: Res<BonusAssets>,
) {
    // Load the images from the assets
    let left_image_id = egui_context.add_image(snake_assets.head.clone());
    let right_image_id = egui_context.add_image(bonus_assets.bonus.clone());

    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.add_space(50.0); // Space above the content

        // Create a horizontal layout
        ui.horizontal(|ui| {
            // Left Column
            ui.vertical_centered(|ui| {
                ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                    left_image_id,
                    egui::vec2(128.0, 128.0), // Size of the image
                )));
            }); // Ensure this column takes 1/3 of width
*/