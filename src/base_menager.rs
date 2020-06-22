use gdnative::NativeClass;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{GodotString, Node2D, PackedScene, ResourceLoader, StaticBody2D, Vector2};
use std::collections::VecDeque;

#[derive(Debug, NativeClass)]
#[inherit(Node2D)]
pub struct BaseMenager {
    sprite_width: f32,
    sprite_height: f32,
    current_end_pos: Vector2,
    base_scene: Option<PackedScene>,
    current_tiles: VecDeque<StaticBody2D>,
}

#[methods]
impl BaseMenager {
    pub fn _init(mut _owner: Node2D) -> Self {
        BaseMenager {
            sprite_width: 336.0,
            sprite_height: 112.0,
            current_end_pos: Vector2::new(0.0, -112.0),
            base_scene: None,
            current_tiles: VecDeque::new(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: Node2D) {
        // Load Base scene and cast it to PackedScene.
        let base_scene = ResourceLoader::godot_singleton()
            .load(
                "res://scenes/Base.tscn".into(),
                GodotString::from_str("PackedScene"),
                false,
            ) // <-- Option<Resource>
            .and_then(|res| res.cast::<PackedScene>()); // <-- Option<PackedScene>

        match base_scene {
            None => godot_print!("Failure loading base"),
            _ => self.base_scene = base_scene,
        }
    }

    #[export]
    pub unsafe fn add_base(&mut self, mut owner: Node2D) {
        match &self.base_scene {
            Some(scene) => {
                // Get base scene instance and cast it to StaticBody2D.
                let instance = scene.instance(0).and_then(|x| x.cast::<StaticBody2D>());
                if let Some(mut ins) = instance {
                    ins.set_position(self.current_end_pos);
                    // Add StaticBody2D to the game.
                    owner.add_child(Some(ins.to_node()), false);
                    // Update current position of spawner.
                    self.current_end_pos += Vector2::new(self.sprite_width, 0.0);
                    self.current_tiles.push_back(ins);
                } else {
                    godot_print!("Problem with casting baseScene to StaticBody2D");
                }
            }
            None => godot_print!("Problem with loading base scene."),
        }
    }

    // Position of right-bottom corner of the last tile in current_tiles. 
    #[export]
    pub fn get_current_end_position(&mut self, _owner: Node2D) -> Vector2 {
        self.current_end_pos
    }

    // Position of right-bottom corner of first tile in current_tiles.
    pub fn get_position_to_remove(&self, _owner: Node2D) -> Vector2 {
        match self.current_tiles.front() {
            Some(tile) => unsafe { tile.get_global_position() + Vector2::new(self.sprite_width, 0.0) },
            None => {
                godot_print!("There are no ground tiles.");
                Vector2::new(0.0, 0.0)
            }
        }
    }

    pub fn remove_base(&mut self, _owner: Node2D) {
      unsafe {self.current_tiles.pop_front().unwrap().queue_free()};
    }
}
