use gdnative::api::StaticBody2D;
use gdnative::prelude::{methods, NativeClass};

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
pub struct Base;

#[methods]
impl Base {
    pub fn new(mut _owner: &StaticBody2D) -> Self {
        Base
    }

    #[export]
    pub fn _on_notifier_screen_exited(&self, owner: &StaticBody2D) {
        owner.queue_free();
    }
}
