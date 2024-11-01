//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    game::bonus::BonusAssets, game::music::GameplayMusic, game::snake::SnakeAssets,
    game::window::LevelAssets, screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        show_loading_screen.run_if(in_state(Screen::Loading)),
    );

    app.add_systems(
        Update,
        continue_to_title_screen.run_if(in_state(Screen::Loading).and_then(all_assets_loaded)),
    );
}

fn show_loading_screen(mut egui_context: EguiContexts) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        // Adding padding around the panel
        ui.add_space(50.0);

        ui.vertical_centered(|ui| {
            // Highscore title with larger font size
            ui.add_space(80.0); // Adjust space above the title if needed
            ui.label(
                egui::RichText::new("Loading...")
                    .size(60.0) // Larger font size for highscore title
                    .strong(),
            );

            // Additional space between the title and score entries
            ui.add_space(30.0);
        });
    });
}

fn continue_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn all_assets_loaded(
    snake_assets: Option<Res<SnakeAssets>>,
    bonus_assets: Option<Res<BonusAssets>>,
    level_assets: Option<Res<LevelAssets>>,
    gameplay_music: Option<Res<GameplayMusic>>,
) -> bool {
    snake_assets.is_some()
        && bonus_assets.is_some()
        && level_assets.is_some()
        && gameplay_music.is_some()
}
