use gdnative::prelude::{godot_print, methods, NativeClass, Ref};
use gdnative::{
    api::*,
    core_types::{Variant, VariantType},
    prelude::PropertyUsage,
    prelude::{ClassBuilder, ExportInfo, Signal, SignalArgument},
};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct BaseManager {
    base_template: Option<Ref<PackedScene>>,
    sprite_height: f32,
    sprite_width: f32,
    end_position: f32,
}

#[methods]
impl BaseManager {
    pub fn new(_owner: &Node2D) -> BaseManager {
        BaseManager {
            base_template: None,
            sprite_height: 112.,
            sprite_width: 336.,
            end_position: 0.,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        self.base_template = ResourceLoader::godot_singleton()
            .load("res://scenes/Base.tscn", "", false)
            .and_then(|scene| {
                unsafe { scene.assume_safe().cast::<PackedScene>() }.map(|x| x.claim())
            });

        match &self.base_template {
            None => godot_print!("Could not load child scene. Check name."),
            _ => {}
        }
    }

    #[export]
    pub fn control_spawning(&mut self, owner: &Node2D, x_end: f32) {
        while x_end > self.end_position {
            self.spawn_one(owner, self.end_position, -self.sprite_height);
            self.end_position += self.sprite_width;
        }
    }

    #[export]
    fn spawn_one(&mut self, owner: &Node2D, x: f32, y: f32) {
        match self.base_template {
            Some(ref base_obj) => {
                let base = unsafe {
                    // unsafe because `assume_safe` function using.
                    base_obj
                        .assume_safe()
                        // Get instance of `PackedScene`.
                        .instance(0)
                        // Can be casted to `StaticBody2D` but `Node2D` is enough.
                        .and_then(|node| node.assume_safe().cast::<Node2D>())
                        .expect("Could not create base instance.")
                };
                base.set_position(euclid::Vector2D::new(x, y));

                // Add base to manager.
                owner.add_child(base, false);
            }
            None => print!("Base template error."),
        }
    }
}
