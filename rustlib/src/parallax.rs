// Parallax needs to be constantly scrolling, this could be a trait implemented in uncontrolled
use gdnative::{prelude::*, api::{Camera2D}};


#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Parallax {}

impl Parallax {
    fn new(_owner: &Node2D) -> Self {
        Parallax {}
    }
}

#[methods]
impl Parallax {
    #[method]
    fn _ready(&mut self, #[base] owner: &Node2D) {
        owner.set_physics_process(true)
    }

    #[method] 
    fn _process(&mut self, #[base] owner: &Node2D, _delta: f32) {
        // Get Camera Node
        let camera:TRef<Camera2D> = unsafe { // boo
            owner.get_node("/root/World/Camera2D") // If we move the camera node (or any node) all Node retrievals need to be refactored <sigh>
                .unwrap()
                .assume_safe()
                .cast::<Camera2D>()
                .unwrap()
        };

        let cam_pos: f32 = camera.position().y - 135.0;
        
        // Get Current position of background children
        let bk1: TRef<Sprite> = unsafe {
            owner.get_node("ConceptBkgParallax01")
                .unwrap()
                .assume_safe()
                .cast::<Sprite>()
                .unwrap()
        };

        let bk1_pos = bk1.position();

        let bk2: TRef<Sprite> = unsafe {
            owner.get_node("ConceptBkgParallax02")
                .unwrap()
                .assume_safe()
                .cast::<Sprite>()
                .unwrap()
        };
     
        let bk2_pos = bk2.position();

        godot_print!("{:?} {:?}", bk1_pos, bk2_pos);

        // Calculate distance
        let cam_to_bk1: f32 = bk1.position().y - cam_pos;
        let cam_to_bk2: f32 = bk2.position().y - cam_pos;

        if f32::abs(cam_to_bk1) > 270.0 * 2.0 {
            // Move the further background to the front
            godot_print!("{} {} {}", bk1_pos.y, bk2_pos.y, cam_pos);

            bk1.set_position(Vector2::new(bk1_pos.x, bk1_pos.y - f32::signum(cam_to_bk1) * 2.0 * 270.0));
        }
        else if f32::abs(cam_to_bk2) > 270.0 * 2.0 {
            godot_print!("{} {} {}", bk1_pos.y, bk2_pos.y, cam_pos);
        
            bk2.set_position(Vector2::new(bk2_pos.x, bk2_pos.y - f32::signum(cam_to_bk1) * 2.0 * 270.0));
        }

    }
}