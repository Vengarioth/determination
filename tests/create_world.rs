extern crate determination;

use determination::*;

#[test]
pub fn create_world() {
    let mut world = World::new();

    world.add_body(Body::new(
        Vec2::new(
            F32::from_i32(0),
            F32::from_i32(0)
        ),
        F32::from_i32(0),
        BodyType::Dynamic
    ));
}
