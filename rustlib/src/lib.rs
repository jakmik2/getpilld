mod uncontrolled;
mod controlled;
mod projectile;

use gdnative::prelude::*;

use crate::uncontrolled::*;
use crate::controlled::*;
use crate::projectile::*;

fn init(handle: InitHandle) {
    handle.add_class::<Uncontrolled>();
    handle.add_class::<Controlled>();
    handle.add_class::<Projectile>();
}

godot_init!(init);