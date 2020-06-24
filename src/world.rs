use crate::base_manager::BaseManager;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{Camera2D, Instance, NativeClass, Node2D, NodePath, RigidBody2D, Vector2};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct World {
    screen_size: Vector2,
    crabby: Option<RigidBody2D>,
    camera: Option<Camera2D>,
    // A reference to a GodotObject with a Rust NativeClass attached.
    base_manager: Instance<BaseManager>,
}

#[methods]
impl World {
    pub fn _init(_owner: Node2D) -> Self {
        World {
            screen_size: Vector2::zero(),
            crabby: None,
            camera: None,
            base_manager: Instance::new(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: Node2D) {
        self.screen_size = owner.get_viewport().unwrap().get_size(); // Vector2::new(480.0, 720.0);
        self.crabby = owner
            .get_node(NodePath::from_str("./Player"))
            .and_then(|n| n.cast::<RigidBody2D>());
        self.camera = owner
            .get_node(NodePath::from_str("./Camera2D"))
            .and_then(|n| n.cast::<Camera2D>());

        self.camera
            .map(|mut cam| cam.set_offset(Vector2::new(-0.25 * self.screen_size.x, 0.0)));

        let base_manager = owner
            .get_node(NodePath::from_str("./BaseManager"))
            .and_then(|n| n.cast::<Node2D>());

        match base_manager {
            Some(base) => {
                // Downcast a Godot base class to a NativeScript instance -- Instance<BaseManager>.
                self.base_manager = Instance::try_from_unsafe_base(base)
                    .expect("Failure to downcast Node2D to BaseMenager");
            }
            None => godot_print!("Problem with loading BaseManager node."),
        }
    }

    #[export]
    unsafe fn _physics_process(&self, _owner: Node2D, _delta: f64) {
        let camera_x = self
            .crabby
            .expect("There is no crab!")
            .get_global_position()
            .x;

        let mut camera_offset = 0.0;
        match self.camera {
            Some(mut cam) => {
                let cam_y = cam.get_global_position().y;
                cam.set_global_position(Vector2::new(camera_x, cam_y));
                camera_offset = cam.get_offset().x;
            }
            None => godot_print!("There is no camera."),
        }

        // Add base tile while camera is moving.
        let current_base_position = self
            .base_manager
            .map_mut_aliased(|manager, owner| manager.get_position_to_add(owner))
            .expect("Can't call menager's function: `get_position_to_add`");

        if current_base_position.x < camera_x + (self.screen_size.x + camera_offset) {
            // Call function from BaseMenager.
            self.base_manager
                .map_mut_aliased(|manager, owner| manager.add_base(owner))
                .expect("Can't call menager's function: `add_base`");
        }

        // Removing tiles when they are out of view.
        //
        //        tile       +----------+z
        //        to         |          |
        //        remove     | what     |
        //          +        | camera   |
        //          |        | see      |
        //          v        |          |
        //    +-----------+ +-----------+
        //    |           | ||         ||
        //    |  tile 1   | ||  tile 2 ||
        //    |           | |-----------+
        //    +-----------+ +----------+
        //    ^  sprite   ^  ^
        //    |  width    |  |
        //    +< ------- >+  |
        //    +-----------+  +
        //                ^  camera view
        //                |  left corner
        //                |  x position
        //    base_position
        //       _to_remove

        let base_position_to_remove = self
            .base_manager
            .map_mut_aliased(|manager, owner| manager.get_position_to_remove(owner))
            .expect("Can't call menager's function: `get_position_to_remove`");

        if base_position_to_remove.x < camera_x + camera_offset {
            self.base_manager
                .map_mut_aliased(|manager, owner| manager.remove_base(owner))
                .expect("Can't call menager's function: `remove_base`");
        }
    }
}
