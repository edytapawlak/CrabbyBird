use gdnative::api::{Node, StaticBody2D};
use gdnative::prelude::{methods, NativeClass};
use gdnative::Ref;

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
pub struct Pipe;

#[methods]
impl Pipe {
    pub fn new(mut _owner: &StaticBody2D) -> Self {
        Pipe
    }

    #[export]
    fn _ready(&mut self, owner: &StaticBody2D) {
        owner.set_collision_layer(2); // 2^1
    }

    #[export]
    pub fn _on_pipe_screen_exited(&self, owner: &StaticBody2D) {
        // Remove pipe.
        owner.queue_free();
    }

    #[export]
    pub unsafe fn _on_middle_body_entered(&self, _owner: &StaticBody2D, body: Ref<Node>) {
        body.assume_safe().emit_signal("pass_pipe", &[]);
    }
}
