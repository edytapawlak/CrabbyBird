use gdnative::NativeClass;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{GodotString, Node2D, NodePath, StaticBody2D, Vector2, VisibilityNotifier2D};

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
pub struct Pipe {
    sprite_width: f32,
    sprite_height: f32,
    notifier: Option<VisibilityNotifier2D>,
}

#[methods]
impl Pipe {
    pub fn _init(mut _owner: StaticBody2D) -> Self {
        Pipe {
            sprite_width: 336.0,
            sprite_height: 112.0,
            notifier: None,
        }
      }
    
    #[export]
    pub unsafe fn _ready(&mut self, owner: StaticBody2D) {
      godot_print!("let us set notifier in redi");
      self.notifier = owner
      .get_node(NodePath::from_str("./Notifier"))
      .and_then(|n| n.cast::<VisibilityNotifier2D>());
    }  

    #[export]
    pub unsafe fn _on_Notifier_screen_exited(&self, mut owner: StaticBody2D) {
      owner.queue_free();
      godot_print!("pipe removed");
    }

    }

