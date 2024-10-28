use bevy::{
    ecs::{system::RunSystemOnce as _, world::Command},
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use crate::{
    asset_tracking::LoadResource,
    demo::movement::{MovementController, ScreenWrap},
    screens::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input.in_set(AppSet::RecordInput),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

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
    player_assets: Res<PlayerAssets>,
) {
    // Spawn the player character using a simple PNG without animation.
    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: player_assets.ducky.clone(),
            transform: Transform::from_scale(Vec2::splat(1.0).extend(1.0)), // Adjust size as needed
            ..Default::default()
        },
        MovementController {
            max_speed: config.max_speed,
            ..default()
        },
        ScreenWrap,
        StateScoped(Screen::Gameplay),
    ));
}

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
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct PlayerAssets {
    #[dependency]
    pub ducky: Handle<Image>,
}

impl PlayerAssets {
    pub const PATH_DUCKY: &'static str = "images/SnakeHead.png"; // Use your PNG path here
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                PlayerAssets::PATH_DUCKY,
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve the pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
