use gdnative::prelude::*;

use crate::{skills::*, projectile::Projectile};


pub struct SkillSystem {
    skills: Vec<Box<dyn Skill>>
}

impl Default for SkillSystem {
    fn default() -> Self {
        let mut v: Vec<Box<dyn Skill>> = Vec::new();

        v.push(Box::new(TestSkill::new()));

        SkillSystem {
            skills: v
        }
    }
}

impl SkillSystem {
    // Operation of Skills will be managed
    pub fn activate(&self, owner: &KinematicBody2D) -> () {
        for skill in &self.skills {
            Projectile::shoot(owner, &*skill.get_image())
        }
    }
}