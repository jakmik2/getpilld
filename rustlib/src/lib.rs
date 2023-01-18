mod uncontrolled;
mod controlled;
mod projectile;
mod skill_system;
mod skills;

use gdnative::prelude::*;

use crate::uncontrolled::*;
use crate::controlled::*;
use crate::projectile::*;
use crate::skill_system::*;
use crate::skills::*;

fn init(handle: InitHandle) {
    handle.add_class::<Uncontrolled>();
    handle.add_class::<Controlled>();
    handle.add_class::<Projectile>();
}

godot_init!(init);