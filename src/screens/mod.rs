//! The game's main screen states and transitions between them.
mod death;
mod gameplay;
mod highscore;
mod loading;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        highscore::plugin,
        gameplay::plugin,
        death::plugin,
        loading::plugin,
        title::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Loading,
    Title,
    HighScore,
    Gameplay,
    Death,
}
