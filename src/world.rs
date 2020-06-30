use crate::base_manager::BaseManager;
use crate::pipe_manager::PipeManager;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{Instance, NativeClass, Node2D, NodePath, Vector2};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct World {
    // A reference to a GodotObject with a Rust NativeClass attached.
    base_manager: Instance<BaseManager>,
    pipe_manager: Instance<PipeManager>,
}

#[methods]
impl World {
    pub fn _init(_owner: Node2D) -> Self {
        World {
            base_manager: Instance::new(),
            pipe_manager: Instance::new(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: Node2D) {
        let screen_size = owner.get_viewport().unwrap().get_size(); // Vector2::new(480.0, 720.0);

        let base_manager = owner
            .get_node(NodePath::from_str("./BaseManager"))
            .and_then(|n| n.cast::<Node2D>());
        base_manager.map(|mut man| man.set_global_position(Vector2::new(0.0, screen_size.y)));

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
        pipe_manager.map(|mut man| man.set_global_position(Vector2::new(0.0, screen_size.y)));

        match pipe_manager {
            Some(manager) => {
                // Downcast a Godot base class to a NativeScript instance -- Instance<PipeManager>.
                self.pipe_manager = Instance::try_from_unsafe_base(manager)
                    .expect("Failure to downcast Node2D to PipeManager");
            }
            None => godot_print!("Problem with loading PipeManager node."),
        }
    }

    pub unsafe fn manage_world(&self, control_position: f32, camera_x_range: (f32, f32)) {
        // Base management.
        self.base_manager
            .map_mut_aliased(|manager, owner| manager.manage_base(owner, camera_x_range))
            .expect("Can't call menager's function: `manage_base`");

        // Pipes management.
        self.pipe_manager
            .map_mut_aliased(|manager, owner| manager.manage_pipes(owner, control_position))
            .expect("Can't call menager's function: `manage_pipes`");
    }
}
