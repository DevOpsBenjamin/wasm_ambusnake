use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler}, window::PrimaryWindow,
};

use crate::{
    asset_tracking::LoadResource,
    game::level::{BOX_SIZE, BOX_COUNT_WIDTH,BOX_COUNT_HEIGHT},
    screens::Screen
};


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub enum SnakeSegment {
    # [default]Head,
    Body,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}
impl Position {
    pub fn to_trasnform(&self, window_width: f32, window_height: f32) -> Transform {
        let box_width_size = window_width/(BOX_COUNT_WIDTH as f32);
        let box_height_size = window_height/(BOX_COUNT_HEIGHT as f32);

        // Calculate the center offset for translation
        let center_offset_x = box_width_size / 2.0;
        let center_offset_y = box_height_size / 2.0;

        let screen_pos_x = (self.x as f32 * box_width_size) - (window_width / 2.0);
        let screen_pos_y =(window_height / 2.0) - (self.y as f32 * box_height_size);

        Transform {
            translation: Vec3::new(
                screen_pos_x + center_offset_x, // Shift X to center the origin
                screen_pos_y - center_offset_y, // Shift Y to center the origin
                0.0,
            ),
            scale: Vec3::new(box_width_size/(BOX_SIZE as f32), box_height_size/(BOX_SIZE as f32), 1.0),
            ..Default::default()
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Index(pub usize);

pub(super) fn plugin(app: &mut App) {
    app.register_type::<SnakeSegment>();
    app.register_type::<Position>();
    app.register_type::<Index>();
    app.load_resource::<SnakeAssets>();

    //HANDLE SNAKE
    app.add_systems(OnEnter(Screen::Gameplay), init_snake);
    app.add_systems(OnExit(Screen::Gameplay), delete_snake);
}

fn init_snake(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    snake_assets: Res<SnakeAssets>,
) {
    // Get the window size for coordinate conversion
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();
    spawn_segment( 
        &mut commands,        
        snake_assets.head.clone(),
        SnakeSegment::Head,
        Position { x: 12, y: 7},
        Index(0),
        window_width,
        window_height
    );
}

fn delete_snake(
    mut commands: Commands,
    query: Query<Entity, With<SnakeSegment>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_segment(
    commands: &mut Commands,
    segment_text: Handle<Image>,
    segment_type: SnakeSegment,
    initial_position: Position,
    index: Index,
    window_width: f32,
    window_height: f32
) {
    commands.spawn((
        segment_type,
        initial_position.clone(),
        index,
        SpriteBundle {
            texture: segment_text,
            transform: initial_position.to_trasnform(window_width, window_height),
            ..Default::default()
        },
    ));
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
