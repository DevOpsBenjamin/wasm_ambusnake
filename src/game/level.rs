use bevy::prelude::*;

pub const BOX_SIZE: i16 = 64;
pub const BG_WIDTH: i16 = 1600;
pub const BG_HEIGHT: i16 = 896;
pub const BOX_COUNT_WIDTH: i16 = BG_WIDTH / BOX_SIZE;
pub const BOX_COUNT_HEIGHT: i16 = BG_HEIGHT / BOX_SIZE;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Position>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Position {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
impl Position {
    pub fn world_transform(&self, window_width: f32, window_height: f32) -> Transform {
        let box_width_size = window_width / (BOX_COUNT_WIDTH as f32);
        let box_height_size = window_height / (BOX_COUNT_HEIGHT as f32);

        // Calculate the center offset for translation
        let center_offset_x = box_width_size / 2.0;
        let center_offset_y = box_height_size / 2.0;

        let screen_pos_x = (self.x as f32 * box_width_size) - (window_width / 2.0);
        let screen_pos_y = (window_height / 2.0) - (self.y as f32 * box_height_size);

        Transform {
            translation: Vec3::new(
                screen_pos_x + center_offset_x, // Shift X to center the origin
                screen_pos_y - center_offset_y, // Shift Y to center the origin
                self.z as f32,
            ),
            scale: Vec3::new(
                box_width_size / (BOX_SIZE as f32),
                box_height_size / (BOX_SIZE as f32),
                1.0,
            ),
            ..Default::default()
        }
    }
}
