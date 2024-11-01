use bevy::prelude::*;

use crate::{game::score::ScoreManager, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Death), spawn_highscore_screen);
}

fn spawn_highscore_screen(mut commands: Commands, query_score: Query<&ScoreManager>) {
    // Return early if there is no single `MoveManager` component
    let score_manager = match query_score.get_single() {
        Ok(score_manager) => score_manager,
        Err(_) => return,
    };

    commands
        .ui_root()
        .insert(StateScoped(Screen::Death))
        .with_children(|children| {
            children.header("YOU LET PATIENT DIE");

            // Display the player's score
            children.label(format!("Score: {}", score_manager.current_score));
            children.input("NAME?");

            children.button("Back").observe(enter_title_screen);
        });
}

fn enter_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
