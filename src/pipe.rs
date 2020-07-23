use gdnative::init::{ClassBuilder, Signal};
use gdnative::NativeClass;
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{GodotString, Node, NodePath, StaticBody2D, VisibilityNotifier2D};

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
#[register_with(Self::register_signals)]
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
        owner.set_collision_layer(2); // 2^1
                                      //owner.set_collision_mask(0);
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "pipe_passed",
            args: &[],
        });
    }

    #[export]
    pub unsafe fn _on_notifier_screen_exited(&self, mut owner: StaticBody2D) {
        owner.queue_free();
    }

    #[export]
    pub unsafe fn _on_middle_body_entered(&self, _owner: StaticBody2D, mut body: Node) {
        //godot_print!("sukcesik! {:?}", body);
        // owner.emit_signal(GodotString::from_str("pipe_passed"), &[]);
        body.emit_signal(GodotString::from_str("pass_pipe"), &[]);
    }
}
