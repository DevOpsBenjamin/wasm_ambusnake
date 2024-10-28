use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq, Eq, Reflect)]
pub struct Pos
{
    pub x: i16,
    pub y: i16
}
impl Pos {
    pub(crate) fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}