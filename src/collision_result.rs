use macroquad::prelude::Vec2;

// Represents the side in which a ball has collided against.
pub enum CollisionResult {
    Top,
    Bottom,
    Left,
    Right,
    Paddle(Vec2),
}
