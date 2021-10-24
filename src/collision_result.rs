use macroquad::prelude::Vec2;
pub enum CollisionResult {
    Top,
    Bottom,
    Left,
    Right,
    Paddle(Vec2),
}
