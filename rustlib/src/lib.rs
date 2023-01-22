mod uncontrolled;
mod player;
mod projectile;
mod skill_system;
mod skills;
mod parallax;
mod constants;
mod camera;

use gdnative::prelude::*;

use crate::uncontrolled::*;
use crate::player::*;
use crate::projectile::*;
use crate::camera::*;
use crate::parallax::*;

fn init(handle: InitHandle) {
    handle.add_class::<Uncontrolled>();
    handle.add_class::<Player>();
    handle.add_class::<Projectile>();
    handle.add_class::<CameraController>();
    handle.add_class::<Parallax>();
}

godot_init!(init);