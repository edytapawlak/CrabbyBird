use gdnative::api::{Input, RigidBody2D};
use gdnative::prelude::{methods, NativeClass, Vector2};
use std::f64::consts::PI;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Player {
    jump_speed: f32,
<<<<<<< HEAD
    max_facing_angle: f64, // Maximal angle bird can faces up, in degrees.
=======
    max_facing_angle: f32, // Maximal facing angle in degrees.
    jump_animation: Option<AnimatedSprite>,
    puff_animation: Option<AnimatedSprite>,
>>>>>>> 7e2e2f8f0eb27acdecd45be78d133af260176236
}

#[methods]
impl Player {
    pub fn new(mut _owner: &RigidBody2D) -> Self {
        Player {
            jump_speed: 500.0,
            max_facing_angle: -30.0,
<<<<<<< HEAD
=======
            jump_animation: None,
            puff_animation: None,
>>>>>>> 7e2e2f8f0eb27acdecd45be78d133af260176236
        }
    }

    #[export]
<<<<<<< HEAD
    unsafe fn _ready(&mut self, owner: &RigidBody2D) {
        // Setting player in the center of the screen
=======
    unsafe fn _ready(&mut self, mut owner: RigidBody2D) {
        // Set player in the center of the screen
>>>>>>> 7e2e2f8f0eb27acdecd45be78d133af260176236
        let size = owner.get_viewport_rect().size;
        owner.set_position(Vector2::new(size.width / 2., size.height / 2.));
        // Set jump animation
        self.jump_animation = owner
            .get_node(NodePath::from_str("./AnimatedSprite"))
            .and_then(|node| node.cast::<AnimatedSprite>());
        // Set puff animation
        self.puff_animation = owner
            .get_node(NodePath::from_str("./PuffAnimation"))
            .and_then(|node| node.cast::<AnimatedSprite>());
    }

    unsafe fn flap(&mut self, owner: &RigidBody2D) {
        // Change player velocity y component to make him jump.
        owner.set_linear_velocity(Vector2::new(owner.linear_velocity().x, -self.jump_speed));
        // Rotate player anti-clockwise when jumping.
        owner.set_angular_velocity(-PI);
        
        // Start flying animation.
        self.jump_animation
            .map(|mut anim| anim.play(GodotString::from_str("jump"), true));

        // Play and show jump smoke.
        self.puff_animation
            .map(|mut anim| anim.play(GodotString::from_str("default"), true));
        self.puff_animation.map(|mut anim| anim.show());
    }

    #[export]
    unsafe fn _physics_process(&mut self, owner: &RigidBody2D, _delta: f64) {
        // Input
        // Flap if space is pressed
<<<<<<< HEAD
        let input = Input::godot_singleton();
        if Input::is_action_pressed(&input, "ui_select") {
=======
        if event
            .expect("Invalid input")
            .is_action_pressed(GodotString::from_str("ui_flap"), false)
        {
>>>>>>> 7e2e2f8f0eb27acdecd45be78d133af260176236
            self.flap(owner);
        }

        // Asure that player can't face up more than max facing_angle
<<<<<<< HEAD
        let actual_rotation = owner.rotation_degrees();
=======
        let actual_rotation = owner.get_rotation_degrees();
        let max_facing_angle = self.max_facing_angle as f64;
>>>>>>> 7e2e2f8f0eb27acdecd45be78d133af260176236

        if actual_rotation < self.max_facing_angle {
            owner.set_rotation_degrees(self.max_facing_angle);
            owner.set_angular_velocity(0.0);
        }
        // Set angular velocity when falling.
        if owner.linear_velocity().y > 0.0 {
            owner.set_angular_velocity(PI / 2.0);
        }
    }

    // Function connected with animation_finished() event in PuffAnimation child.
    #[export]
    unsafe fn _on_puff_animation_finished(&self, _owner: RigidBody2D) {
      // Hide jump smoke
      self.puff_animation
          .map(|mut anim| anim.hide());
    }
}