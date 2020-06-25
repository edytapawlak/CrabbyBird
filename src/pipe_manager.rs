use gdnative::NativeClass;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{GodotString, Node2D, PackedScene, ResourceLoader, StaticBody2D, Vector2};

#[derive(Debug, NativeClass)]
#[inherit(Node2D)]
pub struct PipeManager {
    sprite_width: f32,
    sprite_height: f32,
    pipe_scene: Option<PackedScene>,
}

#[methods]
impl PipeManager {
    pub fn _init(mut _owner: Node2D) -> Self {
        PipeManager {
            sprite_width: 336.0,
            sprite_height: 112.0,
            pipe_scene: None,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: Node2D) {
        // Load Base scene and cast it to PackedScene.
        godot_print!("Pipe ready");
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
        self.add_pipe(owner);
    }

  
    #[export]
    pub fn add_pipe(&mut self, mut owner: Node2D) {
        match &self.pipe_scene {
            Some(scene) => {
                // Get base scene instance and cast it to StaticBody2D.
                let instance = scene
                    .instance(0)
                    .and_then(|x| unsafe { x.cast::<StaticBody2D>() });
                if let Some(mut ins) = instance {
                    unsafe {
                        ins.set_position(Vector2::new(500.0, 0.0));
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