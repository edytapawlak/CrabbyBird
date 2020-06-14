use gdnative::{
    godot_error, godot_gdnative_init, godot_gdnative_terminate, godot_nativescript_init,
};

mod game_state;
mod player;

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<game_state::GameState>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
