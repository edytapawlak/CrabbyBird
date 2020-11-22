use gdnative::prelude::{godot_init, InitHandle};

mod base;
mod base_manager;
mod pipe;
mod pipe_manager;
mod player;
mod world;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<world::World>();
    handle.add_class::<base_manager::BaseManager>();
    handle.add_class::<pipe_manager::PipeManager>();
    handle.add_class::<base::Base>();
    handle.add_class::<pipe::Pipe>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
