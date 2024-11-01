use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    window::PrimaryWindow,
};

use crate::{asset_tracking::LoadResource, game::level::Position, screens::Screen, AppSet};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<SnakeSegment>();
    app.load_resource::<SnakeAssets>();

    //HANDLE SNAKE
    app.add_systems(OnEnter(Screen::Gameplay), init_snake);
    app.add_systems(OnExit(Screen::Gameplay), delete_snake);
    app.add_systems(
        Update,
        snake_grower
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Gameplay)),
    );
}

fn snake_grower(
    commands: Commands,
    snake_assets: Res<SnakeAssets>,
    mut snake_query: Query<&mut SnakeManager>,
    query_segments: Query<&SnakeSegment>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Return early if there is no single `SnakeManager` component
    let mut snake_manager = match snake_query.get_single_mut() {
        Ok(snake_manager) => snake_manager,
        Err(_) => return,
    };

    if snake_manager.should_grow {
        let idx = query_segments.iter().count();
        snake_manager.should_grow = false;
        add_body(commands, window_query, snake_assets, idx);
    }
}

fn init_snake(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    snake_assets: Res<SnakeAssets>,
) {
    commands.spawn(SnakeManager { should_grow: false });
    // Get the window size for coordinate conversion
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();
    spawn_segment(
        &mut commands,
        snake_assets.head.clone(),
        SnakeSegment { idx: 0 },
        Position { x: 12, y: 7, z: 1 },
        window_width,
        window_height,
    );
    add_body(commands, window_query, snake_assets, 1);
}

fn add_body(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    snake_assets: Res<SnakeAssets>,
    idx: usize,
) {
    // Get the window size for coordinate conversion
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();
    //SPAWN BODY
    spawn_segment(
        &mut commands,
        snake_assets.body.clone(),
        SnakeSegment { idx },
        Position {
            x: -50,
            y: -50,
            z: 1,
        },
        window_width,
        window_height,
    );
}

fn delete_snake(
    mut commands: Commands,
    query_segment: Query<Entity, With<SnakeSegment>>,
    query_manager: Query<Entity, With<SnakeManager>>,
) {
    for entity in query_segment.iter() {
        commands.entity(entity).despawn();
    }
    for entity in query_manager.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_segment(
    commands: &mut Commands,
    segment_texture: Handle<Image>,
    segment: SnakeSegment,
    initial_position: Position,
    window_width: f32,
    window_height: f32,
) {
    commands.spawn((
        segment,
        initial_position,
        SpriteBundle {
            texture: segment_texture,
            transform: initial_position.world_transform(window_width, window_height),
            ..Default::default()
        },
    ));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct SnakeManager {
    pub should_grow: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct SnakeSegment {
    pub idx: usize,
}

//ASSET ZONE
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
