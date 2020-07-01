use gdnative::NativeClass;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{GodotString, Node2D, PackedScene, ResourceLoader, StaticBody2D, Vector2};
use std::collections::VecDeque;

#[derive(Debug, NativeClass)]
#[inherit(Node2D)]
pub struct BaseManager {
    sprite_width: f32,
    sprite_height: f32,
    base_scene: Option<PackedScene>,
    current_tiles: VecDeque<StaticBody2D>,
}

#[methods]
impl BaseManager {
    pub fn _init(mut _owner: Node2D) -> Self {
        BaseManager {
            sprite_width: 336.0,
            sprite_height: 112.0,
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

    // Position of right-bottom corner of the last tile in current_tiles.
    fn get_position_to_add(&mut self, _owner: Node2D) -> Vector2 {
        match self.current_tiles.back() {
            Some(tile) => unsafe {
                Vector2::new(
                    tile.get_global_position().x + self.sprite_width,
                    -self.sprite_height,
                )
            },
            None => {
                // There are no ground tiles.
                Vector2::new(0.0, -self.sprite_height)
            }
        }
    }

    // Position of right-bottom corner of first tile in current_tiles.
    fn get_position_to_remove(&self, _owner: Node2D) -> Vector2 {
        match self.current_tiles.front() {
            Some(tile) => unsafe {
                tile.get_global_position() + Vector2::new(self.sprite_width, 0.0)
            },
            None => {
                // There are no ground tiles.
                Vector2::new(self.sprite_width, 0.0)
            }
        }
    }

    fn add_base(&mut self, mut owner: Node2D) {
        match &self.base_scene {
            Some(scene) => {
                // Get base scene instance and cast it to StaticBody2D.
                let instance = scene
                    .instance(0)
                    .and_then(|x| unsafe { x.cast::<StaticBody2D>() });
                if let Some(mut ins) = instance {
                    unsafe {
                        ins.set_position(self.get_position_to_add(owner));
                        ins.set_collision_layer(3);
                        // Add StaticBody2D to the game.
                        owner.add_child(Some(ins.to_node()), false);
                    }
                    // Update current tiles list.
                    self.current_tiles.push_back(ins);
                } else {
                    godot_print!("Problem with casting baseScene to StaticBody2D");
                }
            }
            None => {
                godot_print!("Problem with loading base scene.");
            }
        }
    }

    fn remove_base(&mut self, _owner: Node2D) {
        // Prevent removing first tile to avoid a gap when the game starts.
        if self.current_tiles.len() > 1 {
            // Tolerate unwrap here, because we checked that
            // there are at least two elements in `current_tiles`
            unsafe { self.current_tiles.pop_front().unwrap().queue_free() };
        }
    }

    pub fn manage_base(&mut self, owner: Node2D, camera_x_range: (f32, f32)) {
        // Add base tile if it is needed.
        let current_base_position = self.get_position_to_add(owner);
        if current_base_position.x < camera_x_range.1 {
            self.add_base(owner);
        }

        // Removing tiles when they are out of view.
        let base_position_to_remove = self.get_position_to_remove(owner);
        if base_position_to_remove.x < camera_x_range.0 {
            self.remove_base(owner);
        }
    }
}
