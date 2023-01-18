use gdnative::prelude::*;

use crate::skills::*;


pub struct SkillSystem {
    skills: Vec<Box<dyn Skill>>,
    timers: Vec<f32>
}

impl SkillSystem {
    pub fn new() -> Self {
        let mut sksys = SkillSystem {
            skills: Vec::new(),
            timers: Vec::new()
        };

        // Add Skills
        sksys.add_skill(Box::new(TestSkill::new()));

        sksys
    }

    fn add_skill(&mut self, skill: Box<dyn Skill>) -> () {
        self.skills.push(skill);
        self.timers.push(0.0);
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