use gdnative::prelude::{methods, NativeClass, TRef, Vector2};
use gdnative::{
    api::*,
    godot_print,
    prelude::{Instance, Unique},
};

use crate::base_manager::BaseManager;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct World {
    base_manager: Instance<BaseManager, Unique>,
    screen_size: (f32, f32),
}

#[methods]
impl World {
    pub fn new(_owner: &Node2D) -> Self {
        World {
            base_manager: Instance::new(),
            screen_size: (0.0, 0.0),
        }
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
    fn _ready(&mut self, owner: &Node2D) {
        let size = owner.get_viewport_rect().size;
        self.screen_size = (size.width, size.height);

        // Set camera offset to 1/4 of screen width.
        let camera_offset = { Vector2::new(-0.25 * self.screen_size.0, 0.0) };
        // Set camera offset.
        self.get_camera(owner).set_offset(camera_offset);

        // Base manager.
        let base_manager = owner
            .get_node("./BaseManager")
            .and_then(|m| unsafe { m.assume_unique().cast::<Node2D>() });
        match base_manager {
            Some(base) => {
                // Downcast a Godot base class to a NativeScript instance.
                self.base_manager =
                    Instance::try_from_base(base).expect("Can't downcast to BaseManager");
            }
            None => {
                godot_print!("Problem with loading BaseManager node.");
            }
        }
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

        // Get camera view right bound.
        let camera_x_end = {
            let camera_x_start = new_position.x + camera.offset().x;
            camera_x_start + self.screen_size.0
        };

        // Base management.
        self.base_manager
            .map_mut(|manager, owner| manager.control_spawning(owner.as_ref(), camera_x_end))
            .expect("Can't call menager's function: `manage_base`");
    }
}
