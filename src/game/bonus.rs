use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    window::PrimaryWindow,
};
use rand::Rng;

use crate::{
    asset_tracking::LoadResource,
    game::{
        difficulty::Difficulty,
        level::{Position, BOX_COUNT_HEIGHT, BOX_COUNT_WIDTH},
        score::ScoreManager,
        snake::{SnakeManager, SnakeSegment},
    },
    screens::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BonusAssets>();

    //HANDLE Bonus
    app.add_systems(OnExit(Screen::Gameplay), delete_bonus);
    app.add_systems(
        Update,
        (bonus_colision, bonus_spawner)
            .chain()
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Gameplay)),
    );
}

fn bonus_colision(
    mut commands: Commands,
    query_bonus: Query<(Entity, &Position), With<Bonus>>,
    query_snake: Query<(&Position, &SnakeSegment)>,
    mut query_diff: Query<&mut Difficulty>,
    mut query_score: Query<&mut ScoreManager>,
    mut snake_query: Query<&mut SnakeManager>,
) {
    // Return early if there is no single `Difficulty` component
    let mut difficulty = match query_diff.get_single_mut() {
        Ok(difficulty) => difficulty,
        Err(_) => return,
    };

    // Return early if there is no single `SnakeManager` component
    let mut snake_manager = match snake_query.get_single_mut() {
        Ok(snake_manager) => snake_manager,
        Err(_) => return,
    };

    // Return early if there is no single `MoveManager` component
    let mut score_manager = match query_score.get_single_mut() {
        Ok(score_manager) => score_manager,
        Err(_) => return,
    };

    // Find the head segment with idx == 0, return early if not found
    let head_position = match query_snake.iter().find_map(|(position, segment)| {
        if segment.idx == 0 {
            Some(position)
        } else {
            None
        }
    }) {
        Some(position) => position,
        None => return,
    };

    // Logic using head_position
    for (entity, bonus_position) in query_bonus.iter() {
        // Replace this with your custom collision logic
        if head_position.x == bonus_position.x && head_position.y == bonus_position.y {
            commands.entity(entity).despawn(); // Despawn the bonus on collision with the head
            score_manager.add_point(difficulty.score_per_bonus);
            difficulty.check_level_trigger(score_manager.current_score);
            snake_manager.should_grow = true;
        }
    }
}

fn bonus_spawner(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    bonus_assets: Res<BonusAssets>,
    bonus_query: Query<Entity, With<Bonus>>,
    query_diff: Query<&Difficulty>,
) {
    // Get the window size for coordinate conversion
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    // Return early if there is no single `Difficulty` component
    let difficulty = match query_diff.get_single() {
        Ok(difficulty) => difficulty,
        Err(_) => return,
    };

    // Count the number of bonuses currently on screen
    let bonus_count = bonus_query.iter().count();

    // If the number of bonuses is less than the difficulty's allowed count, spawn a new one
    if bonus_count < difficulty.bonus_count {
        spawn_new(
            &mut commands,
            bonus_assets.bonus.clone(),
            window_width,
            window_height,
        );
    }
}

fn spawn_new(
    commands: &mut Commands,
    bonus_texture: Handle<Image>,
    window_width: f32,
    window_height: f32,
) {
    let bonus_pos = get_random_position();
    commands.spawn((
        Bonus,
        bonus_pos,
        SpriteBundle {
            texture: bonus_texture,
            transform: bonus_pos.world_transform(window_width, window_height),
            ..Default::default()
        },
    ));
}

fn get_random_position() -> Position {
    let mut rng = rand::thread_rng();
    Position {
        x: rng.gen_range(0..BOX_COUNT_WIDTH),
        y: rng.gen_range(0..BOX_COUNT_HEIGHT),
        z: 0,
    }
}

fn delete_bonus(mut commands: Commands, query: Query<Entity, With<Bonus>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Bonus;

//ASSET ZONE
#[derive(Resource, Asset, Reflect, Clone)]
pub struct BonusAssets {
    #[dependency]
    pub bonus: Handle<Image>,
}
impl BonusAssets {
    pub const PATH_BONUS: &'static str = "images/SnakeBonus.png"; // Use your PNG path here
}
impl FromWorld for BonusAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bonus: assets.load_with_settings(
                BonusAssets::PATH_BONUS,
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve the pixel art style.
                    settings.sampler = ImageSampler::linear();
                },
            ),
        }
    }
}
