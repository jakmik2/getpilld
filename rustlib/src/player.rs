use crate::skill_system::*;
use crate::constants::*;

use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    skills: SkillSystem
}

impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            skills: SkillSystem::new()
        }
    }
}

#[methods]
impl Player {
    #[method]
    fn _process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
        self.skills.activate(owner, delta);
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
		let input: &Input = Input::godot_singleton();
		
		let mut velocity: Vector2 = input.get_vector("ui_left",
                                                     "ui_right",
                                                     "ui_up",
                                                     "ui_down",
                                                     1.0);

		velocity.x = (input.get_action_strength("ui_right", false) - input.get_action_strength("ui_left", false)).clamp(-1.0, 1.0) as f32;
		velocity.y = (input.get_action_strength("ui_down", false) - input.get_action_strength("ui_up", false)).clamp(-1.0, 1.0) as f32;

        // Add minimum velocity
        velocity.y -= MIN_VELOCITY * delta;

		owner.move_and_collide(velocity, true, true, false);
    }
}