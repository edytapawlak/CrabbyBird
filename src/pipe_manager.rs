use gdnative::NativeClass;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{GodotString, Node, Node2D, PackedScene, ResourceLoader, StaticBody2D, Vector2};
use rand::{thread_rng, Rng};

#[derive(Debug, NativeClass)]
#[inherit(Node2D)]
pub struct PipeManager {
    last_pipe_position: Vector2,
    pipe_scene: Option<PackedScene>,
    maximal_sprite_height: f32, // Maximal pipe height.
    minimal_sprite_height: f32, // Minimal pipe height.
    pipe_offset: f32,           // Half of space between up and down pipe.
    pipe_density: f32,
}

#[methods]
impl PipeManager {
    pub fn _init(mut _owner: Node2D) -> Self {
        PipeManager {
            last_pipe_position: Vector2::new(0.0, 0.0), // Default value which means that
            // pipe generation hasn't started yet.
            pipe_scene: None,
            maximal_sprite_height: 640.0,
            minimal_sprite_height: 50.0,
            pipe_offset: 70.0,
            pipe_density: 300.0,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: Node2D) {
        // Load Pipe scene and cast it to PackedScene.
        let pipe_scene = ResourceLoader::godot_singleton()
            .load(
                "res://scenes/Pipe.tscn".into(),
                GodotString::from_str("PackedScene"),
                false,
            ) // <-- Option<Resource>
            .and_then(|res| res.cast::<PackedScene>()); // <-- Option<PackedScene>

        match pipe_scene {
            None => godot_print!("Failure loading pipe"),
            _ => self.pipe_scene = pipe_scene,
        };
    }

    fn add_pipe(&mut self, mut owner: Node2D, screen_bottom_margin: f32) {
        match &self.pipe_scene {
            Some(scene) => {
                // Get pipe scene instance and cast it to StaticBody2D.
                let instance = scene
                    .instance(0)
                    .and_then(|x| unsafe { x.cast::<StaticBody2D>() });

                if let Some(mut ins) = instance {
                    unsafe {
                        // Calculate range of the pipe y position.
                        let screen_height = owner.get_global_position().y;
                        let top_margin =
                            -(screen_height - (self.minimal_sprite_height + self.pipe_offset));
                        let bottom_margin = -(screen_bottom_margin
                            + (self.minimal_sprite_height + self.pipe_offset));

                        // Choose random y position in given range.
                        let mut rng = thread_rng();
                        self.last_pipe_position += Vector2::new(self.pipe_density, 0.0);

                        // Update last pipe of the manager
                        // Top and bottom margins are negative, so order is reverse.
                        self.last_pipe_position.y = rng.gen_range(top_margin, bottom_margin);

                        // Set pipe position and add it to a scene.
                        ins.set_position(self.last_pipe_position);
                        owner.add_child(Some(ins.to_node()), false);
                    }
                } else {
                    godot_print!("Problem with casting Pipe scene to StaticBody2D");
                }
            }
            None => {
                godot_print!("Problem with loading Pipe scene.");
            }
        }
    }

    pub unsafe fn manage_pipes(
        &mut self,
        owner: Node2D,
        control_position: f32,
        bottom_margin: f32,
    ) {
        // Pipe management

        // Check if there are any pipes.
        // 0.0 is default x position which means there aren't any pipes yet.
        if self.last_pipe_position.x == 0.0 {
            self.last_pipe_position.x = control_position;
        }
        if (control_position - self.last_pipe_position.x) > self.pipe_density {
            self.add_pipe(owner, bottom_margin);
        }
    }

    pub fn remove_all_pipes(&mut self, owner: Node2D) {
        // Remove old pipes.
        unsafe {
            for pipe in owner.get_children().iter() {
                pipe.try_to_object::<Node>()
                    .and_then(|mut n| Some(n.queue_free()));
            }
        }
        self.last_pipe_position = Vector2::new(0.0, 0.0); // Default value which means that
                                                          // pipe generation hasn't started yet.
    }
}
