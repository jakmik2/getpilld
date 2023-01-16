use gdnative::prelude::*;

use crate::projectile::*;

pub trait Skill {
    fn get_image(&self) -> &str;
}

pub struct TestSkill {
    bullet_image: String
}

impl Skill for TestSkill {
    fn get_image(&self) -> &str {
        &self.bullet_image.as_str()
    }
}

impl TestSkill {
    pub fn new() -> Self {
        TestSkill { bullet_image: "res://sprites/bullet.png".to_string() }
    }
}
