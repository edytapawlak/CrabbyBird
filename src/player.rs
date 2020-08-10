use gdnative::api::{Input, RigidBody2D};
use gdnative::prelude::{methods, NativeClass, Vector2};
use std::f64::consts::PI;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Player {
    jump_speed: f32,
    max_facing_angle: f64, // Maximal angle bird can faces up, in degrees.
}

#[methods]
impl Player {
    pub fn new(mut _owner: &RigidBody2D) -> Self {
        Player {
            jump_speed: 500.0,
            max_facing_angle: -30.0,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: &RigidBody2D) {
        // Setting player in the center of the screen
        let size = owner.get_viewport_rect().size;
        owner.set_position(Vector2::new(size.width / 2., size.height / 2.));
    }

    unsafe fn flap(&mut self, owner: &RigidBody2D) {
        // Change player velocity y component to make him jump.
        owner.set_linear_velocity(Vector2::new(owner.linear_velocity().x, -self.jump_speed));
        // Rotate player anti-clockwise when jumping.
        owner.set_angular_velocity(-PI);
    }

    #[export]
    unsafe fn _physics_process(&mut self, owner: &RigidBody2D, _delta: f64) {
        // Input
        // Flap if space is pressed
        let input = Input::godot_singleton();
        if Input::is_action_pressed(&input, "ui_select") {
            self.flap(owner);
        }

        // Asure that player can't face up more than max facing_angle
        let actual_rotation = owner.rotation_degrees();

        if actual_rotation < self.max_facing_angle {
            owner.set_rotation_degrees(self.max_facing_angle);
            owner.set_angular_velocity(0.0);
        }
        // Set angular velocity when falling.
        if owner.linear_velocity().y > 0.0 {
            owner.set_angular_velocity(PI / 2.0);
        }
    }
}
