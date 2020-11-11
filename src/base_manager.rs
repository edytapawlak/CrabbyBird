use gdnative::prelude::{godot_print, methods, NativeClass, Ref};
use gdnative::{api::*, TRef};

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
    fn _ready(&mut self, owner: &Node2D) {
        self.base_template = load_scene("res://scenes/Base.tscn", |scene| Some(scene.claim()));
        match &self.base_template {
            Some(_scene) => godot_print!("Loaded child scene successfully!"),
            None => godot_print!("Could not load child scene. Check name."),
        }
        self.control_spawning(owner, 600.)
    }

    #[export]
    fn control_spawning(&mut self, owner: &Node2D, x_end: f32) {
        while x_end > self.end_position {
            self.spawn_one(owner, self.end_position, -self.sprite_height);
            self.end_position += self.sprite_width;
        }
    }

    #[export]
    fn spawn_one(&mut self, owner: &Node2D, x: f32, y: f32) {
        if let Some(base_obj) = self.base_template.take() {
            let base_obj = unsafe { base_obj.assume_safe() };
            let base = base_obj
                .instance(0)
                .and_then(|node| {
                    let node = unsafe { node.assume_safe() };
                    node.cast::<Node2D>()
                })
                .expect("Could not create base instance.");
            base.set_position(euclid::Vector2D::new(x, y));

            owner.add_child(base, false);
            self.base_template.replace(base_obj.claim());
        }
    }
}

pub fn load_scene<F, T>(name: &str, mut f: F) -> Option<T>
where
    F: FnMut(TRef<PackedScene>) -> Option<T>,
{
    let scene = ResourceLoader::godot_singleton().load(name, "PackedScene", false)?;
    let scene = unsafe { scene.assume_safe() };
    let packed_scene = scene.cast::<PackedScene>()?;

    f(packed_scene)
}
