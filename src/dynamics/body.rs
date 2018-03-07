use ::math::fixed::F32;
use ::math::vec2::Vec2;

pub enum BodyType {
    /// Zero mass, zero velocity, manually moved
    Static,
    /// Zero mass, non-zero velocity set by user, moved by solver
    Kinematic,
    /// Positive mass, non-zero velocity determined by forces, moved by solver
    Dynamic,
}

pub struct Body {
    position: Vec2,
    angle: F32,
    body_type: BodyType,
}

impl Body {
    pub fn new(
        position: Vec2,
        angle: F32,
        body_type: BodyType,
    ) -> Body {
        Body {
            position: position,
            angle: angle,
            body_type: body_type,
        }
    }
}
