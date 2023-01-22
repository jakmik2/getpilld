use gdnative::{prelude::*, api::Camera2D};

use crate::constants::*;

#[derive(NativeClass)]
#[inherit(Camera2D)]
pub struct CameraController {}

impl CameraController {
    pub fn new(_owner: &Camera2D) -> Self {
        CameraController {  }
    }
}

#[methods]
impl CameraController {
    #[method]
    fn _process(&mut self, #[base] owner: &Camera2D, delta: f32) {
        let mut pos: Vector2 = owner.position();
        pos.y -= MIN_VELOCITY * delta;
        owner.set_position(pos);
    }
}