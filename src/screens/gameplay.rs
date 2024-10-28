//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    game::music::*,
    asset_tracking::LoadResource, demo::level::spawn_level as spawn_level_command,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);

    app.load_resource::<GameplayMusic>();
    app.add_systems(OnEnter(Screen::Gameplay), play_gameplay_music);
    app.add_systems(OnExit(Screen::Gameplay), stop_music);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Gameplay).and_then(input_just_pressed(KeyCode::Escape)))
    );
    /* 
    app.add_systems(
        Update,        
        game_update
            .run_if(in_state(Screen::Gameplay))
    );
fn game_update(mut next_screen: ResMut<NextState<Screen>>) {
    //INGAME
}
*/
}

fn spawn_level(mut commands: Commands) {
    commands.add(spawn_level_command);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
