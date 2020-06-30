use crate::base_manager::BaseManager;
use crate::pipe_manager::PipeManager;
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
    pipe_manager: Instance<PipeManager>,
}

#[methods]
impl World {
    pub fn _init(_owner: Node2D) -> Self {
        World {
            screen_size: Vector2::zero(),
            crabby: None,
            camera: None,
            base_manager: Instance::new(),
            pipe_manager: Instance::new(),
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
        base_manager.map(|mut man| man.set_global_position(Vector2::new(0.0, self.screen_size.y)));

        match base_manager {
            Some(base) => {
                // Downcast a Godot base class to a NativeScript instance -- Instance<BaseManager>.
                self.base_manager = Instance::try_from_unsafe_base(base)
                    .expect("Failure to downcast Node2D to BaseMenager")
            }
            None => godot_print!("Problem with loading BaseManager node."),
        }

        let pipe_manager = owner
            .get_node(NodePath::from_str("./PipeManager"))
            .and_then(|n| n.cast::<Node2D>());
        pipe_manager.map(|mut man| man.set_global_position(Vector2::new(0.0, self.screen_size.y)));

        match pipe_manager {
            Some(manager) => {
                // Downcast a Godot base class to a NativeScript instance -- Instance<PipeManager>.
                self.pipe_manager = Instance::try_from_unsafe_base(manager)
                    .expect("Failure to downcast Node2D to PipeManager");
            }
            None => godot_print!("Problem with loading PipeManager node."),
        }
    }

    #[export]
    unsafe fn _physics_process(&self, _owner: Node2D, _delta: f64) {
        let camera_x = self
            .crabby
            .expect("There is no crab!")
            .get_global_position()
            .x;

        // Start and end x position of what camera can see.
        let mut camera_x_range = (0.0, 0.0);
        match self.camera {
            Some(mut cam) => {
                let cam_y = cam.get_global_position().y;
                cam.set_global_position(Vector2::new(camera_x, cam_y));
                camera_x_range.0 = camera_x + cam.get_offset().x;
                camera_x_range.1 = camera_x_range.0 + self.screen_size.x;
            }
            None => godot_print!("There is no camera."),
        }

        // Base management.
        self.base_manager
            .map_mut_aliased(|manager, owner| {
                manager.manage_base(owner, camera_x_range)
            })
            .expect("Can't call menager's function: `manage_base`");
        // Pipes management.

        // Position behind the camera view in which we decide if new pipe is needed.
        // It is camera position translated by vector (0, screen_width).
        let control_position = self.screen_size.x + camera_x;
        self.pipe_manager
            .map_mut_aliased(|manager, owner| {
                manager.manage_pipes(owner, control_position)
            })
            .expect("Can't call menager's function: `manage_pipes`");
    }
}
