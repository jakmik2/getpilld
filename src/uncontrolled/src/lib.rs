use gdnative::api::Sprite;
use gdnative::prelude::*;

// use gdnative::export::hint::{EnumHint, IntHint, StringHint};

#[derive(gdnative::derive::NativeClass)]
#[inherit(Sprite)]
struct Uncontrolled {

}

impl Uncontrolled {
    fn new(_owner: &Sprite) -> Self {
        Uncontrolled {}
    }
}

#[methods]
impl Uncontrolled {
    #[method]
    fn _ready(&mut self, #[base] owner: &Sprite) {
        owner.set_physics_process(true);
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &Sprite, _delta: f64) {
        godot_print!("Physics process called");
        let mut pos = owner.position();
        pos.x += 1.0;
        owner.set_position(pos);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Uncontrolled>();
}

godot_init!(init);