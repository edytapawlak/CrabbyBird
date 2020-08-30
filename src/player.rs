use gdnative::api::{AnimatedSprite, Input, Node, RigidBody2D};
use gdnative::prelude::{methods, NativeClass, Ref, Vector2};
use std::f64::consts::PI;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Player {
    x_speed: f32,
    jump_speed: f32,
    max_facing_angle: f64, // Maximal facing angle in degrees.
    jump_animation_node: Option<Ref<Node>>,
    puff_animation_node: Option<Ref<Node>>,
}

#[methods]
impl Player {
    pub fn new(_owner: &RigidBody2D) -> Self {
        Player {
            x_speed: 100.0,
            jump_speed: 500.0,
            max_facing_angle: -30.0,
            jump_animation_node: None,
            puff_animation_node: None,
        }
    }

    #[export]

    fn _ready(&mut self, owner: &RigidBody2D) {
        // Set player in the center of the screen
        let size = owner.get_viewport_rect().size;
        owner.set_position(Vector2::new(size.width / 2., size.height / 2.));

        // Find child Nodes
        self.jump_animation_node = owner.get_node("./AnimatedSprite");
        self.puff_animation_node = owner.get_node("./PuffAnimation");
    }

    fn flap(&self, owner: &RigidBody2D) {
        // Change player velocity y component to make him jump.
        owner.set_linear_velocity(Vector2::new(owner.linear_velocity().x, -self.jump_speed));
        // Rotate player anti-clockwise when jumping.
        owner.set_angular_velocity(-PI);

        // Start flying animation.
        self.jump_animation_node
            .and_then(|node| unsafe { node.assume_safe().cast::<AnimatedSprite>() })
            .map(|anim| anim.play("jump", true));

        // Play and show jump smoke.

        self.puff_animation_node
            .and_then(|node| unsafe { node.assume_safe().cast::<AnimatedSprite>() })
            .map(|anim| {
                anim.show();
                anim.play("default", true);
            });
    }

    #[export]
    fn _physics_process(&self, owner: &RigidBody2D, _delta: f64) {
        // Input
        // Flap if space is pressed
        let input = Input::godot_singleton();
        if Input::is_action_pressed(&input, "ui_flap") {
            self.flap(owner);
        }

        // Assure that player can't face up more than max facing_angle
        let actual_rotation = owner.rotation_degrees();
        if actual_rotation < self.max_facing_angle {
            owner.set_rotation_degrees(self.max_facing_angle);
            owner.set_angular_velocity(0.0);
        }
        // Set angular velocity when falling.
        if owner.linear_velocity().y > 0.0 {
            owner.set_angular_velocity(PI / 2.0);
        }

        // Set x of linear velocity.
        owner.set_linear_velocity(Vector2::new(self.x_speed, owner.linear_velocity().y))
    }

    // Function connected with animation_finished() event from PuffAnimation node.
    #[export]
    fn _on_puff_animation_finished(&self, _owner: &RigidBody2D) {
        // Hide jump smoke
        self.puff_animation_node
            .and_then(|node| unsafe { node.assume_safe().cast::<AnimatedSprite>() })
            .map(|anim| anim.hide());
    }
}
