use gdnative::prelude::{godot_print, methods, NativeClass, Ref};
use gdnative::{
    api::*,
    core_types::VariantArray,
    core_types::{Variant},
    TRef,
};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct PipeManager {
    pipe_template: Option<Ref<PackedScene>>,
}

#[methods]
impl PipeManager {
    pub fn new(_owner: &Node2D) -> Self {
        PipeManager {
            pipe_template: None,
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node2D>) {
        self.pipe_template = ResourceLoader::godot_singleton()
            .load("res://scenes/Pipe.tscn", "", false)
            .and_then(|scene| {
                unsafe { scene.assume_safe().cast::<PackedScene>() }.map(|x| x.claim())
            });

        match &self.pipe_template {
            None => godot_print!("Could not load child scene. Check name."),
            _ => {}
        }
        // Get emitter of `pipe_needed` signal.
        let emitter = owner
            .get_parent()
            .and_then(|node| unsafe { Some(node.assume_safe()) })
            .expect("Can't get emitter node.");

        emitter
            .connect(
                "pipe_needed",
                owner,
                "pipe_needed",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
    }

    #[export]
    fn pipe_needed(&mut self, owner: &Node2D, position_x: Variant) {
        godot_print!(
            "Got pipe_needed signal from World on position: {:?}",
            position_x
        );
        self.spawn_one(owner, position_x.to_f64() as f32, 200.)
    }

    #[export]
    fn spawn_one(&mut self, owner: &Node2D, x: f32, y: f32) {
        match self.pipe_template {
            Some(ref pipe_obj) => {
                let pipe = unsafe {
                    // unsafe because `assume_safe` function using.
                    pipe_obj
                        .assume_safe()
                        // Get instance of `PackedScene`.
                        .instance(0)
                        // Can be casted to `StaticBody2D` but `Node2D` is enough.
                        .and_then(|node| node.assume_safe().cast::<Node2D>())
                        .expect("Could not create base instance.")
                };
                pipe.set_position(euclid::Vector2D::new(x, y));

                // Add base to manager.
                owner.add_child(pipe, false);
            }
            None => print!("Pipe template error."),
        }
    }
}
