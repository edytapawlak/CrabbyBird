use gdnative::prelude::{godot_print, methods, NativeClass, Ref};
use gdnative::{api::*, core_types::Variant, core_types::VariantArray, TRef};
use rand::{thread_rng, Rng};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct PipeManager {
    pipe_template: Option<Ref<PackedScene>>,
    maximal_sprite_height: f32, // Maximal pipe height.
    minimal_sprite_height: f32, // Minimal pipe height.
    pipe_offset: f32,           // Half of space between up and down pipe.
}

#[methods]
impl PipeManager {
    pub fn new(_owner: &Node2D) -> Self {
        PipeManager {
            pipe_template: None,
            maximal_sprite_height: 640.0,
            minimal_sprite_height: 50.0,
            pipe_offset: 70.0,
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
    fn pipe_needed(
        &mut self,
        owner: &Node2D,
        position_x: Variant,
        screen_bottom_margin: Variant,
        screen_height: Variant,
    ) {
        godot_print!(
            "Got pipe_needed signal from World on position: {:?}",
            position_x
        );
        // Parse arguments.
        let screen_height = screen_height.to_f64() as f32;
        let screen_margin = screen_bottom_margin.to_f64() as f32;
        // Calculate range of the pipe y position.
        let top_margin = self.minimal_sprite_height + self.pipe_offset;
        let bottom_margin =
            screen_height - (screen_margin + (self.minimal_sprite_height + self.pipe_offset));

        // Choose random y position in given range.
        let mut rng = thread_rng();

        // Top and bottom margins are negative, so order is reverse.
        let y = rng.gen_range(top_margin, bottom_margin);

        self.spawn_one(owner, position_x.to_f64() as f32, y)
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
