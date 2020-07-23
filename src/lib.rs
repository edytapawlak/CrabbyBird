use gdnative::{
    godot_error, godot_gdnative_init, godot_gdnative_terminate, godot_nativescript_init,
};

mod base_manager;
mod game;
mod game_state;
mod pipe;
mod pipe_manager;
mod player;
mod world;

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<game::Game>();
    handle.add_class::<base_manager::BaseManager>();
    handle.add_class::<pipe_manager::PipeManager>();
    handle.add_class::<pipe::Pipe>();
    handle.add_class::<world::World>();
    handle.add_class::<game_state::GameState>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
