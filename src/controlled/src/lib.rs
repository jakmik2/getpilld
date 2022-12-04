use gdnative::api::*;
use gdnative::prelude::*;

// use gdnative::export::hint::{EnumHint, IntHint, StringHint};

#[derive(gdnative::derive::NativeClass)]
#[inherit(KinematicBody2D)]
struct Controlled {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Controlled {
    fn new(_owner: &KinematicBody2D) -> Self {
        Controlled {
            up: false,
            down: false,
            left: false,
            right: false
        }
    }
}

#[methods]
impl Controlled {
    #[method]
    fn _ready(&mut self, #[base] owner: &KinematicBody2D) {
        owner.set_physics_process(true);
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, _delta: f64) {
		let input: &Input = Input::godot_singleton();
		
		let mut velocity: Vector2 = input.get_vector(GodotString::from_str("ui_left"),
                                                     GodotString::from_str("ui_right"),
                                                     GodotString::from_str("ui_up"),
                                                     GodotString::from_str("ui_down"),
                                                     1.0);

		velocity.x = (input.get_action_strength(GodotString::from_str("ui_right"), false) - input.get_action_strength(GodotString::from_str("ui_left"), false)).clamp(-1.0, 1.0) as f32;
		velocity.y = (input.get_action_strength(GodotString::from_str("ui_down"), false) - input.get_action_strength(GodotString::from_str("ui_up"), false)).clamp(-1.0, 1.0) as f32;

		owner.move_and_collide(velocity, true, true, false);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Controlled>();
}

godot_init!(init);