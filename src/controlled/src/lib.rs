use gdnative::api::*;
use gdnative::prelude::*;

// use gdnative::export::hint::{EnumHint, IntHint, StringHint};

#[derive(gdnative::derive::NativeClass)]
#[inherit(KinematicBody2D)]
struct Controlled {
    projectile_scene: Ref<PackedScene, Shared>
}

impl Controlled {
    fn new(_owner: &KinematicBody2D) -> Self {
        Controlled {
            projectile_scene: ResourceLoader::godot_singleton()
                                .load("res://prefabs/Projectile.tscn", "PackedScene", false)
                                .unwrap()
                                .cast::<PackedScene>()
                                .unwrap()
        }
    }
}

#[methods]
impl Controlled {
    // #[method]
    // fn _ready(&mut self, #[base] owner: &KinematicBody2D) {
    //     owner.set_physics_process(true);
    // }

    #[method]
    fn _process(&self, #[base] owner: &KinematicBody2D, _delta: f32) {
        let input: &Input = Input::godot_singleton();
        let shoot = input.get_action_strength("shoot", false) != 0.0;

        if shoot {
            let projectile_scene = unsafe {
                self.projectile_scene.assume_safe()
            };

            let projectile_node = projectile_scene.instance(0).unwrap();

            let projectile = unsafe {
                projectile_node.assume_unique().cast::<KinematicBody2D>().unwrap()
            };
            
            projectile.set_position(Vector2::new(owner.position().x + 10.0, owner.position().y));

            unsafe {
                owner
                .get_parent()
                .unwrap()
                .assume_safe()
                .add_child(projectile, false);
            }
        }
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

fn init(handle: InitHandle) {
    handle.add_class::<Controlled>();
}

godot_init!(init);