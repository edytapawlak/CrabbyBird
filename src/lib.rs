use gdnative::prelude::{godot_init, InitHandle};

mod player;
mod world;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<world::World>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
