use std::f64::consts::PI;

use gdnative::prelude::*;
use crate::projectile::Projectile;

pub trait Skill {
    fn get_image(&self) -> &str;
    fn activate(&self, owner: &KinematicBody2D, timer: f32) -> bool;
}

pub struct TestSkill {
    bullet_image: String,
    frequency: f32
}

impl TestSkill {
    pub fn new() -> Self {
        TestSkill { 
            bullet_image: "res://sprites/bullet.png".to_string(),
            frequency: 2.0
        }
    }
}

impl Skill for TestSkill {
    fn get_image(&self) -> &str {
        &self.bullet_image.as_str()
    }

    fn activate(&self, owner: &KinematicBody2D, timer: f32) -> bool {
        if timer >= self.frequency{
            // Shoot four projectiles, each 90 degrees apart
            Projectile::shoot(owner, &self.bullet_image, 0.0);
            Projectile::shoot(owner, &self.bullet_image, PI / 2.0);
            Projectile::shoot(owner, &self.bullet_image, PI);
            Projectile::shoot(owner, &self.bullet_image, 3.0 * PI / 2.0);

            return true;
        }
        false
    }
}