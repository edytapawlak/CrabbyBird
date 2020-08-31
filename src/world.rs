use gdnative::api::{Camera2D, Node, Node2D};
use gdnative::prelude::{methods, NativeClass, TRef, Vector2};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct World;

#[methods]
impl World {
    pub fn new(_owner: &Node2D) -> Self {
        World
    }

    fn get_crabby(&self, owner: &Node) -> TRef<'_, Node2D> {
        owner
            .get_node("./Player")
            .and_then(|cam| unsafe { cam.assume_safe().cast::<Node2D>() })
            .expect("There is no crabby")
    }

    fn get_camera(&self, owner: &Node) -> TRef<'_, Camera2D> {
        owner
            .get_node("./Camera2D")
            .and_then(|cam| unsafe { cam.assume_safe().cast::<Camera2D>() })
            .expect("There is no camera")
    }

    #[export]
    fn _ready(&self, owner: &Node2D) {
        // Set camera offset to 1/4 of screen width.
        let camera_offset = {
            let screen_width = owner.get_viewport_rect().size.width;
            Vector2::new(-0.25 * screen_width, 0.0)
        };
        // Set camera offset.
        self.get_camera(owner).set_offset(camera_offset);
    }

    #[export]
    fn _physics_process(&self, owner: &Node2D, _delta: f64) {
        // Get TRef to camera Node.
        let camera = self.get_camera(owner);

        // Change only x position of camera to make it follow crabby.
        let new_position = {
            let camera_x = self.get_crabby(owner).global_position().x;
            let camera_y = camera.global_position().y;
            Vector2::new(camera_x, camera_y)
        };
        // Set camera to new position.
        camera.set_global_position(new_position);
    }
}
