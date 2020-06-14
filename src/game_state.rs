use gdnative::{
    godot_error, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{Camera2D, NativeClass, Node2D, NodePath, RigidBody2D, Vector2};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GameState {
    crabby: Option<RigidBody2D>,
    camera: Option<Camera2D>,
}

#[methods]
impl GameState {
    pub fn _init(_owner: Node2D) -> Self {
        GameState {
            crabby: None,
            camera: None,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: Node2D) {
        self.crabby = owner
            .get_node(NodePath::from_str("./Player"))
            .and_then(|n| n.cast::<RigidBody2D>());
        self.camera = owner
            .get_node(NodePath::from_str("./Camera2D"))
            .and_then(|n| n.cast::<Camera2D>());
    }

    #[export]
    unsafe fn _physics_process(&self, _owner: Node2D, _delta: f64) {
        let camera_x = self.crabby.unwrap().get_global_position().x;
        let camera_y = self.camera.unwrap().get_global_position().y;
        match self.camera {
            Some(mut x) => x.set_global_position(Vector2::new(camera_x, camera_y)),
            None => (),
        }
    }
}
