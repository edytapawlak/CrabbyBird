use crate::base_menager::BaseMenager;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{Camera2D, Instance, NativeClass, Node2D, NodePath, RigidBody2D, Vector2};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct World {
    crabby: Option<RigidBody2D>,
    camera: Option<Camera2D>,
    // A reference to a GodotObject with a rust NativeClass attached.
    base_menager: Instance<BaseMenager>,
}

#[methods]
impl World {
    pub fn _init(_owner: Node2D) -> Self {
        World {
            crabby: None,
            camera: None,
            base_menager: Instance::new(),
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
        let base_menager = owner
            .get_node(NodePath::from_str("./BaseSpawner"))
            .and_then(|n| n.cast::<Node2D>());

        match base_menager {
            Some(base) => {
                // Downcast a Godot base class to a NativeScript instance -- Instance<BaseMenager>.
                self.base_menager = Instance::try_from_unsafe_base(base).expect("No way");
            }
            None => godot_print!("Problem with loading BaseMenager node."),
        }
    }

    #[export]
    unsafe fn _physics_process(&self, _owner: Node2D, _delta: f64) {
        let camera_x = self
            .crabby
            .expect("There is no crab!")
            .get_global_position()
            .x;
        let camera_y = self
            .camera
            .expect("There is no camera!")
            .get_global_position()
            .y;
        match self.camera {
            Some(mut x) => x.set_global_position(Vector2::new(camera_x, camera_y)),
            None => (),
        }

        // Add base tile while camera is moving.
        let current_base_position = self
            .base_menager
            .map_mut_aliased(|menager, owner| menager.get_current_end_position(owner))
            .unwrap();
        if current_base_position.x < camera_x + 480.0 {
            self.base_menager
                .map_mut_aliased(|menager, owner| menager.add_base(owner));


        // Removing tiles when they are out of view.
        //
        //        tile       +----------+
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
            .base_menager
            .map_mut_aliased(|menager, owner| menager.get_position_to_remove(owner))
            .unwrap();

        if base_position_to_remove.x < camera_x {
          self.base_menager
          .map_mut_aliased(|menager, owner| menager.remove_base(owner));
        }
        }
    }
}
