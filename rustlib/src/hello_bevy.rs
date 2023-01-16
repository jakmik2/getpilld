use crate::ecs::*;

use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloBevy {
    pub ecs: Ecs,
}

impl HelloBevy {
    fn new(_owner: &Node) -> Self {
        HelloBevy {
            ecs: Ecs::default()
        }
    }
}

#[methods]
impl HelloBevy {
    #[method]
    fn _ready(&mut self) {
        self.ecs.spawn_some_entities();
        let entities = self.ecs.get_entities();
        godot_print!("{}", entities.len());
    }
}
