use gdnative::prelude::*;

use crate::skills::*;


pub struct SkillSystem {
    skills: Vec<Box<dyn CanShoot>>,
    timers: Vec<f32>
}

impl SkillSystem {
    pub fn new() -> Self {
        let mut v: Vec<Box<dyn CanShoot>> = Vec::new();
        v.push(Box::new(TestSkill::new()));

        let mut t: Vec<f32> = Vec::new();
        t.push(0.0);

        SkillSystem {
            skills: v,
            timers: t
        }
    }
}

impl SkillSystem {
    // Operation of Skills will be managed
    pub fn activate(&mut self, owner: &KinematicBody2D, delta: f32) -> () {
        self.update_time(delta);

        for (idx, skill) in self.skills.iter().enumerate() {
            let activated = skill.activate(owner, self.timers[idx]);
            if activated {
                self.timers[idx] = 0.0
            }
        }
    }

    fn update_time(&mut self, val: f32) -> (){
        self.timers.iter_mut().for_each(|x| *x += val);
    }
}