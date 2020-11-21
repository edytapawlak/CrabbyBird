use gdnative::api::StaticBody2D;
use gdnative::{
    godot_print,
    prelude::{methods, NativeClass},
};

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
pub struct Pipe;

#[methods]
impl Pipe {
    pub fn new(mut _owner: &StaticBody2D) -> Self {
        Pipe
    }

    #[export]
    pub fn _on_notifier_screen_exited(&self, owner: &StaticBody2D) {
        owner.queue_free();
    }
}