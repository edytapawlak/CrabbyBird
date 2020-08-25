use gdnative::api::{Node, Node2D};
use gdnative::prelude::{methods, NativeClass, Ref, Vector2};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct World {
    crabby: Option<Ref<Node>>,
    camera: Option<Ref<Node>>,
}

#[methods]
impl World {
    pub fn new(_owner: &Node2D) -> Self {
        World {
            crabby: None,
            camera: None,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        self.crabby = owner.get_node("./Player");
        self.camera = owner.get_node("./Camera2D");
    }

    #[export]
    fn _physics_process(&self, _owner: &Node2D, _delta: f64) {
        // Get crabby x position to make camera follow him.
        let camera_x = self
            .crabby
            .and_then(|node| unsafe { node.assume_safe().cast::<Node2D>() })
            .expect("There is no crab!")
            .global_position()
            .x;

        // Get TRef to camera Node.
        let camera = self
            .camera
            .and_then(|x| unsafe { x.assume_safe().cast::<Node2D>() })
            .expect("There is no camera!");
        // Get camera y position.
        let camera_y = camera.global_position().y;

        // Set camera global position.
        camera.set_global_position(Vector2::new(camera_x, camera_y));
    }
}
