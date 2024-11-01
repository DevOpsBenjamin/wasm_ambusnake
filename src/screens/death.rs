use bevy::prelude::*;

use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Death), spawn_highscore_screen);
}

fn spawn_highscore_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Death))
        .with_children(|children| {
            children.header("YOU LET PATIENT DIE");

            children.button("Back").observe(enter_title_screen);
        });
}

fn enter_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
