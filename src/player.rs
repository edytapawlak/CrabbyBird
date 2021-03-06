use gdnative::init::{ClassBuilder, Signal};
use gdnative::NativeClass;
use gdnative::{godot_error, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods};
use gdnative::{AnimatedSprite, GodotString, InputEvent, Node, NodePath, RigidBody2D, Vector2, CPUParticles2D};
use std::f64::consts::PI;

pub enum PlayerState {
    Flying,
    Flapping,
    Dead,
}

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
// register_with attribute can be used to specify custom register function for node signals and properties
#[register_with(Self::register_signals)]
pub struct Player {
    jump_speed: f32,
    x_speed: f32,
    max_facing_angle: f32, // Maximal facing angle in degrees.
    jump_animation: Option<AnimatedSprite>,
    puff_animation: Option<AnimatedSprite>,
    puf_particles: Option<CPUParticles2D>,
    state: PlayerState,
    default_gravity_scale: f64,
}

#[methods]
impl Player {
    pub fn _init(mut _owner: RigidBody2D) -> Self {
        Player {
            jump_speed: 400.0,
            x_speed: 100.0,
            max_facing_angle: -30.0,
            jump_animation: None,
            puff_animation: None,
            puf_particles: None,
            state: PlayerState::Flying,
            default_gravity_scale: 15.0,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, mut owner: RigidBody2D) {
        // Set player in the center of the screen
        let size = owner.get_viewport_rect().size;
        owner.set_position(Vector2::new(size.width / 2., size.height / 2.));

        owner.set_collision_layer(1); // 2^0

        // We want to set collision with 1 and 2 mask layer. 2^1 + 2^2 = 6
        // 1 for collisions with pieps,
        // 2 for collisions with base.
        owner.set_collision_mask(6);

        // Set jump animation
        self.jump_animation = owner
            .get_node(NodePath::from_str("./AnimatedSprite"))
            .and_then(|node| node.cast::<AnimatedSprite>());
        // Set puff animation
        self.puff_animation = owner
            .get_node(NodePath::from_str("./PuffAnimation"))
            .and_then(|node| node.cast::<AnimatedSprite>());

        self.puf_particles = owner
            .get_node(NodePath::from_str("./PuffParticles"))
            .and_then(|node| node.cast::<CPUParticles2D>());

        owner.set_linear_velocity(Vector2::new(self.x_speed, owner.get_linear_velocity().y));
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "control_start",
            args: &[],
        });

        builder.add_signal(Signal {
            name: "player_collision",
            args: &[],
        });

        builder.add_signal(Signal {
            name: "pass_pipe",
            args: &[],
        });

        builder.add_signal(Signal {
          name: "space_new_game",
          args: &[],
      });
    }

    #[export]
    unsafe fn flap(&mut self, mut owner: RigidBody2D) {
        // Change player velocity y component to make him jump.
        owner.set_linear_velocity(Vector2::new(
            owner.get_linear_velocity().x,
            -self.jump_speed,
        ));
        // Rotate player anti-clockwise when jumping.
        owner.set_angular_velocity(-PI);

        // Start flying animation.
        self.jump_animation
            .map(|mut anim| anim.play(GodotString::from_str("jump"), true));

        // Play and show jump smoke.
        //self.puff_animation
        //    .map(|mut anim| anim.play(GodotString::from_str("default"), true));
        //self.puff_animation.map(|mut anim| anim.show());

        self.puf_particles
          .map(|mut puff| puff.restart());
    }

    unsafe fn fly(&self, mut owner: RigidBody2D, delta: f32) {
        owner.set_gravity_scale(0.0);
        // Set horizontal velocity to move player forward with x_speed.
        // Don't change vertical position.
        owner.set_linear_velocity(Vector2::new(self.x_speed, 0.0));
        let pos = owner.get_global_position();
        // Make player swing a little.
        owner.set_global_position(Vector2::new(pos.x, pos.y + (pos.x * delta).sin()));

        // Start flying animation.
        self.jump_animation
            .map(|mut anim| anim.play(GodotString::from_str("fly"), true));
    }

    unsafe fn dead(&self, mut owner: RigidBody2D) {
        owner.set_linear_velocity(Vector2::new(0.0, owner.get_linear_velocity().y));
        owner.set_collision_mask(4); // 2^2, 2 for collision with base
        self.jump_animation
            .map(|mut anim| anim.play(GodotString::from_str("gameover"), false));
    }

    #[export]
    unsafe fn _input(&mut self, mut owner: RigidBody2D, event: Option<InputEvent>) {
        // Flap if space is pressed
        if event
            .expect("Invalid input")
            .is_action_pressed(GodotString::from_str("ui_flap"), false)
        {
            match self.state {
                PlayerState::Flying => {
                    self.state = PlayerState::Flapping;
                    // Emit signal to World to start generate obstacles.
                    owner.emit_signal(GodotString::from_str("control_start"), &[]);
                    self.flap(owner);
                }
                PlayerState::Flapping => self.flap(owner),
                PlayerState::Dead => {
                  owner.emit_signal(GodotString::from_str("space_new_game"), &[]);
                }
            }
        }
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut owner: RigidBody2D, delta: f64) {
        match self.state {
            PlayerState::Flying => {
                self.fly(owner, delta as f32);
            }
            PlayerState::Flapping => {
                owner.set_gravity_scale(self.default_gravity_scale);
                owner
                    .set_linear_velocity(Vector2::new(self.x_speed, owner.get_linear_velocity().y));
                // Asure that player can't face up more than max facing_angle
                let actual_rotation = owner.get_rotation_degrees();
                let max_facing_angle = self.max_facing_angle as f64;

                if actual_rotation < max_facing_angle {
                    owner.set_rotation_degrees(max_facing_angle);
                    owner.set_angular_velocity(0.0);
                }
                // Set angular velocity when falling.
                if owner.get_linear_velocity().y > 0.0 {
                    owner.set_angular_velocity(PI / 2.0);
                }
            }
            PlayerState::Dead => {
                self.dead(owner);
            }
        }
    }

    // Function connected with animation_finished() event in PuffAnimation child.
    #[export]
    unsafe fn _on_puff_animation_finished(&self, _owner: RigidBody2D) {
        // Hide jump smoke
        self.puff_animation.map(|mut anim| anim.hide());
    }

    #[export]
    unsafe fn _on_player_body_entered(&mut self, mut owner: RigidBody2D, _node: Node) {
        self.state = PlayerState::Dead;
        owner.emit_signal(GodotString::from_str("player_collision"), &[]);
    }

    #[export]
    fn handle_new_game(&mut self, mut owner: RigidBody2D) {
        self.state = PlayerState::Flying;
        unsafe {
            // Activate collisions with pipes.
            owner.set_collision_mask(6);
            // Set player to the center of game screen.
            let size = owner.get_viewport_rect().size;
            owner.set_position(Vector2::new(size.width / 2., size.height / 2.));
            // Set player rotation degree.
            owner.set_rotation_degrees(0.);
            owner.set_angular_velocity(0.);
        }
    }
}
