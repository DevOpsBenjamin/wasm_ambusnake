use bevy::{prelude::*, window::PrimaryWindow};

use std::time::Duration;

use crate::{
    game::{
        difficulty::Difficulty,
        snake::{Position, SnakeSegment},
    },
    screens::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), init_move);
    app.add_systems(OnExit(Screen::Gameplay), delete_move);

    app.add_systems(Update, update_timer.in_set(AppSet::TickTimers));
    app.add_systems(Update, check_input.in_set(AppSet::RecordInput));
    app.add_systems(
        Update,
        (check_movement, apply_transform)
            .chain()
            .in_set(AppSet::Update),
    );
}

/// Update the animation timer.
fn update_timer(time: Res<Time>, mut query: Query<&mut MoveManager>) {
    for mut move_manager in &mut query {
        move_manager.update_timer(time.delta());
    }
}

fn check_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query_move: Query<&mut MoveManager>,
    query_diff: Query<&Difficulty>,
) {
    // Return early if there is no single `MoveManager` component
    let mut move_manager = match query_move.get_single_mut() {
        Ok(move_manager) => move_manager,
        Err(_) => return,
    };
    // Return early if there is no single `Difficulty` component
    let difficulty = match query_diff.get_single() {
        Ok(difficulty) => difficulty,
        Err(_) => return,
    };

    let mut got_input = false;
    // Handle movement direction
    for &(key, arrow, dir) in &DIRECTIONS_KEY {
        if input.pressed(key) || input.pressed(arrow) {
            // If timer is set, use try_add; if not, set direction directly
            if move_manager.timer.is_some() {
                move_manager.try_add(dir);
            } else {
                move_manager.dir = dir;
                move_manager.next_dir = dir;
                got_input = true
            }
        }
    }

    //IF we have no timer we init
    if move_manager.timer.is_none() && got_input {
        move_manager.timer = Some(Timer::from_seconds(
            difficulty.move_duration.as_secs_f32(),
            TimerMode::Repeating,
        ));
    }
}

fn check_movement(
    mut query_move: Query<&mut MoveManager>,
    mut query_snake: Query<(&mut Position, &SnakeSegment)>,
) {
    // Return early if there is no single `MoveManager` component
    let mut move_manager = match query_move.get_single_mut() {
        Ok(move_manager) => move_manager,
        Err(_) => return,
    };

    // Return early if there is not a move timer
    if let Some(timer) = &move_manager.timer {
        if !timer.finished() {
            return; // Exit early if the timer is not finished
        }
    } else {
        return; // Exit early if there is no timer
    }
    //THE NEW MOVE IS THE OLD NEXT DIR
    move_manager.dir = move_manager.next_dir;

    // Collect all segments into a vector and sort by index in ascending order
    let mut segments: Vec<_> = query_snake.iter_mut().collect();
    // Sort by idx value
    segments.sort_by(|(_, idx_a), (_, idx_b)| idx_a.idx.cmp(&idx_b.idx));
    // GetMutHead
    let (head_position, _) = segments.get_mut(0).unwrap();
    // OLD POS
    let mut last_x = head_position.x;
    let mut last_y = head_position.y;

    match move_manager.dir {
        Dir::Left => head_position.x -= 1,
        Dir::Right => head_position.x += 1,
        Dir::Up => head_position.y -= 1,
        Dir::Down => head_position.y += 1,
    }

    for (body_position, _) in segments.iter_mut().skip(1)
    // Skip the head
    {
        // Save the current position
        let curr_x = body_position.x;
        let curr_y = body_position.y;
        // Update the body segment's position to the last position
        body_position.x = last_x;
        body_position.y = last_y;
        // Update last_x and last_y to the current position for the next segment
        last_x = curr_x;
        last_y = curr_y;
    }
}

fn apply_transform(
    window_query: Query<&Window, With<PrimaryWindow>>,
    query_move: Query<&MoveManager>,
    mut segment_query: Query<(&mut Transform, &Position), With<SnakeSegment>>,
) {
    // Return early if there is no single `MoveManager` component
    let move_manager = match query_move.get_single() {
        Ok(move_manager) => move_manager,
        Err(_) => return,
    };

    // Return early if there is not a move timer
    if let Some(timer) = &move_manager.timer {
        if !timer.finished() {
            return; // Exit early if the timer is not finished
        }
    } else {
        return; // Exit early if there is no timer
    }

    //REFRESH SCREEN
    let window = window_query.single();
    let current_width = window.width();
    let current_height = window.height();

    // Apply snake MOVe
    for (mut transform, position) in &mut segment_query {
        let new_transform = position.to_trasnform(current_width, current_height);
        transform.translation = new_transform.translation;
    }
}

fn init_move(mut commands: Commands) {
    commands.spawn(MoveManager::default());
}

fn delete_move(mut commands: Commands, query: Query<Entity, With<MoveManager>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// Define the direction tuple type
type DirectionKey = (KeyCode, KeyCode, Dir);
// Define the directions as a constant
const DIRECTIONS_KEY: [DirectionKey; 4] = [
    (KeyCode::KeyW, KeyCode::ArrowUp, Dir::Up),
    (KeyCode::KeyS, KeyCode::ArrowDown, Dir::Down),
    (KeyCode::KeyA, KeyCode::ArrowLeft, Dir::Left),
    (KeyCode::KeyD, KeyCode::ArrowRight, Dir::Right),
];

// Define the direction tuple type with possible moves
type PossibleDirection = (Dir, Dir, Dir);
// (current_direction, possible_move_1, possible_move_2)
// Define the directions as a constant
const POSSIBLE_DIRECTIONS: [PossibleDirection; 4] = [
    (Dir::Up, Dir::Left, Dir::Right),
    (Dir::Down, Dir::Left, Dir::Right),
    (Dir::Left, Dir::Up, Dir::Down),
    (Dir::Right, Dir::Up, Dir::Down),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum Dir {
    Left,
    #[default]
    Right,
    Up,
    Down,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct MoveManager {
    //DIRECTION
    pub dir: Dir,
    next_dir: Dir,
    timer: Option<Timer>,
}
impl MoveManager {
    /// Update timers if they are set.
    pub fn update_timer(&mut self, delta: Duration) {
        if let Some(timer) = &mut self.timer {
            timer.tick(delta);
        }
    }

    pub fn try_add(&mut self, dir: Dir) {
        // Find the current direction tuple
        if let Some(&(_, possibility_1, possibility_2)) =
            POSSIBLE_DIRECTIONS.iter().find(|&&(d, _, _)| d == self.dir)
        {
            // Check if the proposed direction is one of the possible moves
            if dir == possibility_1 || dir == possibility_2 {
                self.next_dir = dir;
            }
        }
    }
}
