use bevy::{
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use crate::{
    asset_tracking::LoadResource,
    game::pos::Pos,
    //demo::movement::{MovementController, ScreenWrap},
    screens::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Snake>();
    app.load_resource::<SnakeAssets>();

    // Record directional input as movement controls.
    //app.add_systems(
    //    Update,
    //    record_player_directional_input.in_set(AppSet::RecordInput),
    //);
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct Snake
{
    pub head: Pos,
    pub bodies: Vec<Pos>
}
impl Default for Snake{
    fn default() -> Self {
        Self { head: Pos::new(12,8), bodies: Vec::new() }
    }
}

/// A command to spawn the player character.
#[derive(Debug)]
pub struct SpawnPlayer {
    /// See [`MovementController::max_speed`].
    pub max_speed: f32,
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}

fn spawn_player(
    In(config): In<SpawnPlayer>,
    mut commands: Commands,
    player_assets: Res<SnakeAssets>,
) {
    // Spawn the player character using a simple PNG without animation.
    commands.spawn((
        Name::new("Snake"),
        Snake::default(),
        SpriteBundle {
            texture: player_assets.head.clone(),
            transform: Transform::from_scale(Vec2::splat(1.0).extend(1.0)), // Adjust size as needed
            ..Default::default()
        },
        /* 
        MovementController {
            max_speed: config.max_speed,
            ..default()
        },
        ScreenWrap,*/
        StateScoped(Screen::Gameplay),
    ));
}
/*
fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 64.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 64.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 64.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 64.0;
    }

    // Normalize so that diagonal movement has the same speed as horizontal and vertical movement.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}*/

#[derive(Resource, Asset, Reflect, Clone)]
pub struct SnakeAssets {
    #[dependency]
    pub head: Handle<Image>,
    pub body: Handle<Image>,
}

impl SnakeAssets {
    pub const PATH_HEAD: &'static str = "images/SnakeHead.png"; // Use your PNG path here
    pub const PATH_BODY: &'static str = "images/SnakeBody.png"; // Use your PNG path here
}

impl FromWorld for SnakeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            head: assets.load_with_settings(
                SnakeAssets::PATH_HEAD,
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve the pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            body: assets.load_with_settings(
                SnakeAssets::PATH_BODY,
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve the pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
