use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), setup_score_manager);
    app.add_systems(OnEnter(Screen::Gameplay), reset_score);
}

// Setup the `ScoreManager` on entering the Title screen only if it doesn’t already exist
fn setup_score_manager(mut commands: Commands, query: Query<Entity, With<ScoreManager>>) {
    if query.is_empty() {
        commands.spawn(ScoreManager { current_score: 0 });
    }
}

fn reset_score(mut query: Query<&mut ScoreManager>) {
    // Return early if there is no single `MoveManager` component
    let mut score_manager = match query.get_single_mut() {
        Ok(score_manager) => score_manager,
        Err(_) => return,
    };
    score_manager.reset_score();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct ScoreManager {
    pub current_score: usize,
}
impl ScoreManager {
    pub fn add_point(&mut self, point: usize) {
        self.current_score += point;
    }
    pub fn reset_score(&mut self) {
        self.current_score = 0;
    }
}
