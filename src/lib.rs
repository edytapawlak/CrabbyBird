use gdnative::*;

mod player;

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::prelude::InitHandle) {
    handle.add_class::<player::Player>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
