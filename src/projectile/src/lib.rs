use gdnative::api::*;
use gdnative::prelude::*;

#[derive(gdnative::derive::NativeClass)]
#[inherit(KinematicBody2D)]
struct Projectile {
    acceleration: f32,
    max_speed: f32,
    direction: Vector2,
    window_size: Vector2
}

impl Projectile {
    fn new(_owner: &KinematicBody2D) -> Self {
        Projectile {
            acceleration: 1.0,
            max_speed: 10.0,
            direction: Vector2::new(1.0, 0.0),
            window_size: OS::godot_singleton().window_size()
        }
    }
}

#[methods]
impl Projectile {
    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
        let mut velocity = Vector2::new(1.0, 0.0);
        velocity = velocity.move_toward(self.direction * self.max_speed, self.acceleration * delta);
        owner.move_and_collide(velocity, true, true, false);

        if owner.position().x > self.window_size.x
            || owner.position().x < 0.0
            || owner.position().y > self.window_size.y
            || owner.position().y < 0.0
        {
            unsafe {
                owner.assume_unique().queue_free();
            }
        }
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Projectile>();
}

godot_init!(init);