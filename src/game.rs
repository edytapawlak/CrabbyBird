use crate::world::World;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{Camera2D, Instance, NativeClass, Node2D, NodePath, RigidBody2D, Vector2};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    screen_size: Vector2,
    crabby: Option<RigidBody2D>,
    camera: Option<Camera2D>,
    // A reference to a GodotObject with a Rust NativeClass attached.
    world: Instance<World>,
}

#[methods]
impl Game {
    pub fn _init(_owner: Node2D) -> Self {
        Game {
            screen_size: Vector2::zero(),
            crabby: None,
            camera: None,
            world: Instance::new(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: Node2D) {
        self.screen_size = owner
            .get_viewport()
            .expect("Can't get screen size.")
            .get_size(); // Vector2::new(480.0, 720.0);
        self.crabby = owner
            .get_node(NodePath::from_str("./Player"))
            .and_then(|n| n.cast::<RigidBody2D>());

        self.camera = owner
            .get_node(NodePath::from_str("./Camera2D"))
            .and_then(|n| n.cast::<Camera2D>());

        // Set camera offset to 1/4 screen width, to let player see more space in front of him.
        self.camera
            .map(|mut cam| cam.set_offset(Vector2::new(-0.25 * self.screen_size.x, 0.0)));

        let world = owner
            .get_node(NodePath::from_str("./World"))
            .and_then(|n| n.cast::<Node2D>());

        match world {
            Some(w) => {
                // Downcast a Godot base class to a NativeScript instance -- Instance<BaseManager>.
                self.world =
                    Instance::try_from_unsafe_base(w).expect("Failure to downcast Node2D to World")
            }
            None => godot_print!("Problem with loading World node."),
        }
    }

    #[export]
    unsafe fn _physics_process(&self, _owner: Node2D, _delta: f64) {
        let camera_x = self
            .crabby
            .expect("There is no crab!")
            .get_global_position()
            .x;

        let mut camera_x_range = (0.0, 0.0); // Start and end x position of what camera can see.
        match self.camera {
            Some(mut cam) => {
                let cam_y = cam.get_global_position().y;
                cam.set_global_position(Vector2::new(camera_x, cam_y));
                camera_x_range.0 = camera_x + cam.get_offset().x;
                camera_x_range.1 = camera_x_range.0 + self.screen_size.x;
            }
            None => godot_print!("There is no camera."),
        }

        let control_position = self.screen_size.x + camera_x; // Position outside the camera view in which we decide
                                                              // if new pipe is needed.
                                                              // It is camera x position translated by screen_width.

        // Manage world generation.
        self.world
            .map_mut_aliased(|manager, _owner| {
                manager.manage_world(control_position, camera_x_range)
            })
            .expect("Can't call function: `manage_world`");
    }
}
