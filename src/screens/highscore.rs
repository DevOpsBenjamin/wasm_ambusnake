//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::HighScore), spawn_highscore_screen);
}

fn spawn_highscore_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::HighScore))
        .with_children(|children| {
            children.header("HIGHSCORE");
            children.label("Jebik - 42");

            children.button("Back").observe(enter_title_screen);
        });
}

fn enter_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
