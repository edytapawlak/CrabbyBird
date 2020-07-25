use crate::world::World;
use gdnative::init::{ClassBuilder, Signal};
use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{
    Camera2D, CanvasLayer, GodotString, Instance, NativeClass, Node2D, NodePath, RigidBody2D,
    VariantArray, Vector2,
};

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct Game {
    screen_size: Vector2,
    crabby: Option<RigidBody2D>,
    camera: Option<Camera2D>,
    // A reference to a GodotObject with a Rust NativeClass attached.
    world: Instance<World>,
    game_state: Option<CanvasLayer>,
}

#[methods]
impl Game {
    pub fn _init(_owner: Node2D) -> Self {
        Game {
            screen_size: Vector2::zero(),
            crabby: None,
            camera: None,
            world: Instance::new(),
            game_state: None,
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "new_game",
            args: &[],
        });
    }

    #[export]
    unsafe fn _ready(&mut self, mut owner: Node2D) {
        self.screen_size = owner
            .get_viewport()
            .expect("Can't get screen size.")
            .get_size(); // Vector2::new(480.0, 720.0);
        self.crabby = owner
            .get_node(NodePath::from_str("./Player"))
            .and_then(|n| n.cast::<RigidBody2D>());

        self.camera = owner
            .get_node(NodePath::from_str("./Camera2D"))
            .and_then(|n| n.cast::<Camera2D>());

        // Set camera offset to 1/4 screen width, to let player see more space in front of him.
        self.camera
            .map(|mut cam| cam.set_offset(Vector2::new(-0.25 * self.screen_size.x, 0.0)));

        self.game_state = owner
            .get_node(NodePath::from_str("./GameState"))
            .and_then(|n| n.cast::<CanvasLayer>());

        let world = owner
            .get_node(NodePath::from_str("./World"))
            .and_then(|n| n.cast::<Node2D>());
        // If crabby is not None
        if let Some(mut crabby) = self.crabby {
            // Connect `new_game` signal to crabby.
            owner
                .connect(
                    GodotString::from_str("new_game"),
                    Some(crabby.to_object()),
                    GodotString::from_str("handle_new_game"),
                    VariantArray::new(),
                    1,
                )
                .expect("Problem with connecting `new_game` signal to world");
            // If game state is not None
            if let Some(mut game_stat) = self.game_state {
                // Connect pass_pipe signal.
                crabby
                    .connect(
                        GodotString::from_str("pass_pipe"),
                        Some(game_stat.to_object()),
                        GodotString::from_str("notify_pass_pipe"),
                        VariantArray::new(),
                        1,
                    )
                    .expect("Problem with connecting `pass_pipe` signal");

                // Connect game over signal.
                crabby
                    .connect(
                        GodotString::from_str("player_collision"),
                        Some(game_stat.to_object()),
                        GodotString::from_str("game_over"),
                        VariantArray::new(),
                        0,
                    )
                    .expect("Problem with connecting `player_collision` signal");

                // Connect `new_game_pressed` signal to `new_game` function.
                game_stat
                    .connect(
                        GodotString::from_str("new_game_pressed"),
                        Some(owner.to_object()),
                        GodotString::from_str("new_game"),
                        VariantArray::new(),
                        0,
                    )
                    .expect("Problem with connecting `new_game_pressed` signal");

                // Connect `new_game` signal to `game_state`.
                owner
                    .connect(
                        GodotString::from_str("new_game"),
                        Some(game_stat.to_object()),
                        GodotString::from_str("handle_new_game"),
                        VariantArray::new(),
                        1,
                    )
                    .expect("Problem with connecting `new_game` signal to world");
            } else {
                godot_print!("Problem with loading GameState node");
            }

            // If world is not None
            match world {
                Some(w) => {
                    // Downcast a Godot base class to a NativeScript instance -- Instance<BaseManager>.
                    self.world = Instance::try_from_unsafe_base(w)
                        .expect("Failure to downcast Node2D to World");
                    // Connect signal to start generating pipes.
                    crabby
                        .connect(
                            GodotString::from_str("control_start"),
                            Some(w.to_object()),
                            GodotString::from_str("notify_control_start"),
                            VariantArray::new(),
                            1,
                        )
                        .expect("Problem with connecting `control_start` signal");

                    // Connect `new_game` signal to world.
                    owner
                        .connect(
                            GodotString::from_str("new_game"),
                            Some(w.to_object()),
                            GodotString::from_str("handle_new_game"),
                            VariantArray::new(),
                            1,
                        )
                        .expect("Problem with connecting `new_game` signal to world");
                }
                None => godot_print!("Problem with loading World node."),
            }
        } else {
            godot_print!("Problem with loading Player node");
        }
    }

    #[export]
    unsafe fn _physics_process(&self, _owner: Node2D, _delta: f64) {
        let camera_x = self
            .crabby
            .expect("There is no crab!")
            .get_global_position()
            .x;

        let mut camera_x_range = (0.0, 0.0); // Start and end x position of what camera can see.
        match self.camera {
            Some(mut cam) => {
                let cam_y = cam.get_global_position().y;
                cam.set_global_position(Vector2::new(camera_x, cam_y));
                camera_x_range.0 = camera_x + cam.get_offset().x;
                camera_x_range.1 = camera_x_range.0 + self.screen_size.x;
            }
            None => godot_print!("There is no camera."),
        }

        let control_position = self.screen_size.x + camera_x; // Position outside the camera view in which we decide
                                                              // if new pipe is needed.
                                                              // It is camera x position translated by screen_width.

        // Manage world generation.
        self.world
            .map_mut_aliased(|manager, _owner| {
                manager.manage_world(control_position, camera_x_range)
            })
            .expect("Can't call function: `manage_world`");
    }

    #[export]
    fn new_game(&mut self, mut owner: Node2D) {
        unsafe {
            owner.emit_signal(GodotString::from_str("new_game"), &[]);
        }
    }
}
