use gdnative::api::*;
use gdnative::prelude::*;

#[derive(gdnative::derive::NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Projectile {
    acceleration: f32,
    max_speed: f32,
    direction: Vector2,
    window_size: Vector2
}

impl Projectile {
    const PROJECTILE_SCENE: &str = "res://prefabs/Projectile.tscn";
    
    fn new(_owner: &KinematicBody2D) -> Self {
        Projectile {
            acceleration: 10.0,
            max_speed: 100.0,
            direction: Vector2::new(1.0, 0.0),
            window_size: OS::godot_singleton().window_size()
        }
    }

    pub fn at(_owner: &KinematicBody2D, direction: Vector2) -> Self {
        Projectile {
            acceleration: 10.0,
            max_speed: 5.0,
            direction: direction,
            window_size: OS::godot_singleton().window_size()
        }
    }
    
    pub fn shoot(&self, owner: &KinematicBody2D, proj_sprite_path: &str) {
        
        let sprite = unsafe { 
            owner.get_child(0)
                .unwrap()
                .assume_safe()
                .cast::<Sprite>()
                .unwrap()
        };

        sprite.set_texture(
            ResourceLoader::godot_singleton()
                .load(proj_sprite_path, "Texture", false)
                .unwrap()
                .cast::<Texture>()
                .unwrap()
        );

        let process_scene: Ref<PackedScene, Shared> = ResourceLoader::godot_singleton()
                                                                        .load(Projectile::PROJECTILE_SCENE, "PackedScene", false)
                                                                        .unwrap()
                                                                        .cast::<PackedScene>()
                                                                        .unwrap();


        let projectile_scene: TRef<PackedScene> = unsafe {
            process_scene.assume_safe()
        };

        let projectile_node: Ref<Node> = projectile_scene.instance(0).unwrap();

        let projectile: Ref<KinematicBody2D, Unique> = unsafe {
            projectile_node.assume_unique().cast::<KinematicBody2D>().unwrap()
        };

        projectile.set_position(owner.position());

        unsafe {
            owner
            .get_parent()
            .unwrap()
            .assume_safe()
            .add_child(projectile, false)
        }
    }
}

#[methods]
impl Projectile {
    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
        
        godot_print!("{:?}", self.direction);
        let velocity = owner.position().move_toward(self.direction * self.max_speed, self.acceleration * delta);
        owner.move_and_collide(velocity, true, true, false);

        // Destroy self if outside window
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