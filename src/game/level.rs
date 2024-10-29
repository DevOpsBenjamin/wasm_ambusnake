
use bevy::{ecs::world::Command, prelude::*};

use crate::{demo::player::SpawnPlayer, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), init_data);
    app.add_systems(OnExit(Screen::Gameplay), reset_data);
    //
    /* 
    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Gameplay).and_then(input_just_pressed(KeyCode::Escape))));
    */
}

fn init_data(mut commands: Commands) {
    //commands.add(spawn_level_command);
}

fn reset_data(mut commands: Commands) {
    //next_screen.set(Screen::Title);
}


pub const BOX_SIZE: i16 = 64;
pub const BG_WIDTH: i16 = 1600;
pub const BG_HEIGHT: i16 = 896;
pub const BOX_COUNT_WIDTH: i16 = BG_WIDTH / BOX_SIZE;
pub const BOX_COUNT_HEIGHT: i16 = BG_HEIGHT / BOX_SIZE;

pub struct Level
{
    pub running: bool,
    pub score: i16
}