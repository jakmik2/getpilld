use crate::projectile::*;
use crate::skill_system::*;
use crate::skills::*;

use gdnative::api::*;
use gdnative::prelude::*;

// use gdnative::export::hint::{EnumHint, IntHint, StringHint};

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Controlled {
    skills: SkillSystem,
    timer: f32
}

impl Controlled {
    fn new(_owner: &KinematicBody2D) -> Self {
        Controlled {
            skills: SkillSystem::default(),
            timer: 0.0
        }
    }
}

#[methods]
impl Controlled {
    #[method]
    fn _process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
        // let input: &Input = Input::godot_singleton();
        self.timer += delta;
        if self.timer >= 1.0 {
            self.skills.activate(owner);
            self.timer = 0.0;
        }


        // this should be managed by a skills / skill manager instance
        // if input.get_action_strength("shoot", false) != 0.0
        // {
        //     // TODO: This will be delegated to a skill

        //     let angle: f64 = owner.get_local_mouse_position().angle_to_point(owner.position()) as f64;            

        //     let proj_sprite_path: &str = "res://sprites/bullet.png";

        //     Projectile::shoot(owner, proj_sprite_path);
        // }
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, _delta: f64) {
		let input: &Input = Input::godot_singleton();
		
		let mut velocity: Vector2 = input.get_vector("ui_left",
                                                     "ui_right",
                                                     "ui_up",
                                                     "ui_down",
                                                     1.0);

		velocity.x = (input.get_action_strength("ui_right", false) - input.get_action_strength("ui_left", false)).clamp(-1.0, 1.0) as f32;
		velocity.y = (input.get_action_strength("ui_down", false) - input.get_action_strength("ui_up", false)).clamp(-1.0, 1.0) as f32;

		owner.move_and_collide(velocity, true, true, false);
    }
}