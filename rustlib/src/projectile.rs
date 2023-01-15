use std::f32::consts::PI;

use gdnative::api::*;
use gdnative::prelude::*;

#[derive(gdnative::derive::NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Projectile {
    acceleration: f32,
    max_speed: f32,
    rotation: f64,
    timer: f32,
    existence: f32
}

impl Projectile {
    fn new(_owner: &KinematicBody2D) -> Self {
        godot_print!("New");

        Projectile {
            acceleration: 5.0,
            max_speed: 25.0,
            rotation: 0.1, // radians / sec,
            timer: 0.0,
            existence: 2.0
        }
    }
    
    pub fn shoot(owner: &KinematicBody2D, proj_sprite_path: &str, angle: f64) {

        let process_scene: Ref<PackedScene, Shared> = ResourceLoader::godot_singleton()
                                                                        .load("res://prefabs/Projectile.tscn", "PackedScene", false)
                                                                        .unwrap()
                                                                        .cast::<PackedScene>()
                                                                        .unwrap();

        godot_print!("process scene");

        let projectile_scene: TRef<PackedScene> = unsafe {
            process_scene.assume_safe()
        };

        let projectile_node: Ref<Node> = projectile_scene.instance(0).unwrap();

        godot_print!("Instance / unwrap");

        let projectile: Ref<KinematicBody2D, Unique> = unsafe {
            projectile_node.assume_unique().cast::<KinematicBody2D>().unwrap()
        };

        godot_print!("Cast kinematicBody2D");

        projectile.set_position(owner.position());

        // Set direction of projectile
        projectile.rotate(-1.0 * angle - (PI / 3.0) as f64);

        unsafe {
            owner
                .get_parent()
                .unwrap()
                .assume_safe()
                .add_child(projectile, false)
        };
    }
}

#[methods]
impl Projectile {
    #[method]
    fn _process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
        // Countdown on life
        self.existence -= delta;

        // TODO: Destroy self if outside window
        // Currently, destroying self after time limit
        if self.existence < 0.0 {
            owner.queue_free();
        }
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
        // Apply rotation
        owner.rotate(self.rotation);

        // Progress timer
        self.timer += delta;

        // TODO: Optional, should be implemented in skills 
        if self.timer >= 0.1 {
            self.timer = 0.0;
            self.rotation -= 0.02;
        }

        // TODO: Implement collision return and collision function
        owner.move_and_collide(owner.get_transform().a * self.max_speed * self.acceleration * delta, true, true, false);

    }
}