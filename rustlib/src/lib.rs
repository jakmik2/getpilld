mod uncontrolled;
mod controlled;
mod projectile;
mod ecs;
mod hello_bevy;

use gdnative::prelude::*;

use crate::uncontrolled::*;
use crate::controlled::*;
use crate::projectile::*;
use crate::hello_bevy::*;

fn init(handle: InitHandle) {
    handle.add_class::<Uncontrolled>();
    handle.add_class::<Controlled>();
    handle.add_class::<Projectile>();
    handle.add_class::<HelloBevy>();
}

godot_init!(init);