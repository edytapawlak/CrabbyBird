use gdnative::{
    api::*,
    core_types::VariantArray,
    godot_print,
    prelude::{Instance, Unique},
};
use gdnative::{
    core_types::{Variant, VariantType},
    prelude::{
        methods, ClassBuilder, ExportInfo, NativeClass, PropertyUsage, Signal, SignalArgument,
        TRef, Vector2,
    },
};

use crate::base_manager::BaseManager;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct World {
    obstacle_status: bool,
    base_manager: Instance<BaseManager, Unique>,
    screen_size: (f32, f32),
    // Pipe spawning settings.
    pipe_density: f32,
    last_pipe_x: f32,
}

#[methods]
impl World {
    pub fn new(_owner: &Node2D) -> Self {
        World {
            obstacle_status: false,
            base_manager: Instance::new(),
            screen_size: (0.0, 0.0),
            pipe_density: 250.,
            last_pipe_x: 500.0,
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "pipe_needed",
            args: &[SignalArgument {
                name: "position_x",
                default: Variant::from_i64(100),
                export_info: ExportInfo::new(VariantType::I64),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    fn get_crabby(&self, owner: TRef<Node2D>) -> TRef<'_, Node2D> {
        owner
            .get_node("./Player")
            .and_then(|cam| unsafe { cam.assume_safe().cast::<Node2D>() })
            .expect("There is no crabby")
    }

    fn get_camera(&self, owner: TRef<Node2D>) -> TRef<'_, Camera2D> {
        owner
            .get_node("./Camera2D")
            .and_then(|cam| unsafe { cam.assume_safe().cast::<Camera2D>() })
            .expect("There is no camera")
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node2D>) {
        let size = owner.get_viewport_rect().size;
        self.screen_size = (size.width, size.height);

        // Set camera offset to 1/4 of screen width.
        let camera_offset = { Vector2::new(-0.25 * self.screen_size.0, 0.0) };
        // Set camera offset.
        self.get_camera(owner).set_offset(camera_offset);

        // Base manager.
        let base_manager = owner
            .get_node("./BaseManager")
            .and_then(|m| unsafe { m.assume_unique().cast::<Node2D>() });
        match base_manager {
            Some(base) => {
                // Downcast a Godot base class to a NativeScript instance.
                self.base_manager =
                    Instance::try_from_base(base).expect("Can't downcast to BaseManager");
            }
            None => {
                godot_print!("Problem with loading BaseManager node.");
            }
        }

        // Connect game over signal.
        self.get_crabby(owner)
            .connect(
                "game_over",
                owner,
                "handle_game_over",
                VariantArray::new_shared(),
                0,
            )
            .expect("Problem with connecting `game_over` signal");

        // Connect signal to start generating pipes.
        self.get_crabby(owner)
            .connect(
                "control_started",
                owner,
                "handle_control_start",
                VariantArray::new_shared(),
                1,
            )
            .expect("Problem with connecting `control_started` signal");
    }

    #[export]
    fn handle_game_over(&self, _owner: &Node2D) {
        godot_print!("Game Over!")
        // TODO game over.
    }

    #[export]
    fn handle_control_start(&mut self, _owner: &Node2D) {
        // Start obstales generation.
        self.obstacle_status = true;
    }

    #[export]
    fn _physics_process(&mut self, owner: TRef<Node2D>, _delta: f64) {
        // Get TRef to camera Node.
        let camera = self.get_camera(owner);

        // Change only x position of camera to make it follow crabby.
        let new_position = {
            let camera_x = self.get_crabby(owner).global_position().x;
            let camera_y = camera.global_position().y;

            Vector2::new(camera_x, camera_y)
        };

        // Set camera to new position.
        camera.set_global_position(new_position);

        // Get camera view right bound.
        let camera_x_end = {
            let camera_x_start = new_position.x + camera.offset().x;
            camera_x_start + self.screen_size.0
        };

        // Base management.
        self.base_manager
            .map_mut(|manager, owner| manager.control_spawning(owner.as_ref(), camera_x_end))
            .expect("Can't call menager's function: `manage_base`");

        // Emit signal to pipe_manager if pipe is needed.
        if camera_x_end - self.last_pipe_x > self.pipe_density && self.obstacle_status {
            self.last_pipe_x = camera_x_end;
            owner.emit_signal(
                "pipe_needed",
                &[
                    Variant::from_i64(camera_x_end as i64),
                    Variant::from_i64(112.0 as i64), // temporary value. It's height of base image.
                    Variant::from_i64(self.screen_size.1 as i64),
                ],
            );
        }
    }
}
