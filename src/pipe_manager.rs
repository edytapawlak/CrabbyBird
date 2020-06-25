use gdnative::NativeClass;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{GodotString, Node2D, PackedScene, ResourceLoader, StaticBody2D, Vector2};
use rand::{thread_rng, Rng};

#[derive(Debug, NativeClass)]
#[inherit(Node2D)]
pub struct PipeManager {
    pipe_current: Vector2,
    pipe_scene: Option<PackedScene>,
    sprite_height: f32,
    pipe_offset: f32,
}

#[methods]
impl PipeManager {
    pub fn _init(mut _owner: Node2D) -> Self {
        PipeManager {
            pipe_current: Vector2::new(480.0, 0.0),
            pipe_scene: None,
            sprite_height: 320.0,
            pipe_offset: 90.0,
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

    pub fn get_current_pipe_pos(&self, _owner: Node2D) -> Vector2 {
        self.pipe_current
    }

    #[export]
    pub fn add_pipe(
        &mut self,
        mut owner: Node2D,
        pipe_density: f32,
        screen_height: f32,
        screen_bottom_margin: f32,
    ) {
        match &self.pipe_scene {
            Some(scene) => {
                // Get base scene instance and cast it to StaticBody2D.
                let instance = scene
                    .instance(0)
                    .and_then(|x| unsafe { x.cast::<StaticBody2D>() });
                if let Some(mut ins) = instance {
                    let top_margin = -screen_height + self.sprite_height + self.pipe_offset;
                    let bottom_margin =
                        -screen_bottom_margin - self.sprite_height - self.pipe_offset;

                    let mut rng = thread_rng();
                    self.pipe_current += Vector2::new(pipe_density, 0.0);
                    self.pipe_current.y = rng.gen_range(bottom_margin, top_margin);
                    unsafe {
                        ins.set_position(self.pipe_current);
                        owner.add_child(Some(ins.to_node()), false);
                    }
                } else {
                    godot_print!("Problem with casting baseScene to StaticBody2D");
                }
            }
            None => {
                godot_print!("Problem with loading pipe scene.");
            }
        }
    }
}
