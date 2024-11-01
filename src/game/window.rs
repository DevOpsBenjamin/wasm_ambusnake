use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    window::PrimaryWindow,
};

use crate::{
    asset_tracking::LoadResource,
    game::{
        level::{Position, BG_HEIGHT, BG_WIDTH},
        snake::SnakeSegment,
    },
    screens::Screen,
};

#[derive(Default, Resource)]
struct LastWindowSize {
    width: f32,
    height: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct LevelBackGround;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelBackGround>();
    app.load_resource::<LevelAssets>();
    app.insert_resource(LastWindowSize::default());

    app.add_systems(OnEnter(Screen::Gameplay), init_bg);
    app.add_systems(OnExit(Screen::Gameplay), delete_bg);

    //HANDLE RESIZE
    app.add_systems(Update, screen_update);
}

fn screen_update(
    mut last_size: ResMut<LastWindowSize>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut object_query: Query<(&mut Transform, &Position), Without<LevelBackGround>>,
    mut bg_query: Query<&mut Transform, (With<LevelBackGround>, Without<SnakeSegment>)>,
) {
    let window = window_query.single();
    let current_width = window.width();
    let current_height = window.height();

    // Check if the size has changed
    if current_width == last_size.width && current_height == last_size.height {
        return;
    }

    // Update the last known size
    last_size.width = current_width;
    last_size.height = current_height;

    // Apply snake resize
    for (mut transform, position) in &mut object_query {
        let new_transform = position.world_transform(current_width, current_height);
        transform.translation = new_transform.translation;
        transform.scale = new_transform.scale;
    }

    // Apply bg resize
    for mut transform in &mut bg_query {
        transform.scale = Vec3::new(
            current_width / (BG_WIDTH as f32),
            current_height / (BG_HEIGHT as f32),
            1.,
        );
    }
}

fn delete_bg(mut commands: Commands, query: Query<Entity, With<LevelBackGround>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn init_bg(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    snake_assets: Res<LevelAssets>,
) {
    // Get the window size for coordinate conversion
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    commands.spawn((
        LevelBackGround,
        SpriteBundle {
            texture: snake_assets.background.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., -1.),
                scale: Vec3::new(
                    window_width / (BG_WIDTH as f32),
                    window_height / (BG_HEIGHT as f32),
                    1.,
                ),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

//ASSET ZONE
#[derive(Resource, Asset, Reflect, Clone)]
pub struct LevelAssets {
    #[dependency]
    pub background: Handle<Image>,
}
impl LevelAssets {
    pub const PATH_BG: &'static str = "images/SnakeBg.png"; // Use your PNG path here
}
impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load_with_settings(
                LevelAssets::PATH_BG,
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve the pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
