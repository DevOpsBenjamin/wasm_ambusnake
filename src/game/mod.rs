//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

pub mod bonus;
mod difficulty;
mod level;
mod movement;
pub mod music;
mod score;
pub mod snake;
pub mod window;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        bonus::plugin,
        level::plugin,
        snake::plugin,
        movement::plugin,
        difficulty::plugin,
        music::plugin,
        window::plugin,
        score::plugin,
    ));
}
