use gdnative::NativeClass;
use gdnative::{godot_error, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods};
use gdnative::{NodePath, StaticBody2D, VisibilityNotifier2D};

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
pub struct Pipe {
    notifier: Option<VisibilityNotifier2D>,
}

#[methods]
impl Pipe {
    pub fn _init(mut _owner: StaticBody2D) -> Self {
        Pipe { notifier: None }
    }

    #[export]
    pub unsafe fn _ready(&mut self, mut owner: StaticBody2D) {
        self.notifier = owner
            .get_node(NodePath::from_str("./Notifier"))
            .and_then(|n| n.cast::<VisibilityNotifier2D>());
        owner.set_collision_layer(2);
        owner.set_collision_mask(0);
    }

    #[export]
    pub unsafe fn _on_notifier_screen_exited(&self, mut owner: StaticBody2D) {
        owner.queue_free();
    }
}
