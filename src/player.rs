use gdnative::*;
use std::f64::consts::PI;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Player {
    jump_speed: f32,
    facing_angle: f32, // Maximal angle bird can faces up, in degrees.
}

#[methods]
impl Player {
    pub fn _init(mut _owner: RigidBody2D) -> Self {
        Player {
            jump_speed: 500.0,
            facing_angle: -30.0,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, mut owner: RigidBody2D) {
        // Setting player in the center of the screen
        let size = owner.get_viewport_rect().size;
        owner.set_position(Vector2::new(size.width / 2., size.height / 2.));
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
        let animation = owner
            .get_node(NodePath::from_str("./AnimatedSprite"))
            .and_then(|node| node.cast::<AnimatedSprite>());
        // Start flying animation.
        animation
            .expect("AnimationSprite doesnt't exist")
            .play(GodotString::from_str("jump"), true);
        let mut puff = owner
            .get_node(NodePath::from_str("./PuffAnimation"))
            .and_then(|node| node.cast::<AnimatedSprite>())
            .expect("AnimationSprite doesnt't exist");
        // Start flying animation.
        puff.play(GodotString::from_str("default"), true);
        puff.show();

    }

    #[export]
    unsafe fn _input(&mut self, owner: RigidBody2D, event: Option<InputEvent>) {
        // Flap if space is pressed
        if event
            .unwrap()
            .is_action_pressed(GodotString::from_str("ui_select"), false)
        {
            self.flap(owner);
        }
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut owner: RigidBody2D, _delta: f64) {
        // Asure that player can't face up more than max facing_angle
        let actual_rotation = owner.get_rotation_degrees();
        let max_facing_angle = self.facing_angle as f64;

        if actual_rotation < max_facing_angle {
            owner.set_rotation_degrees(max_facing_angle);
            owner.set_angular_velocity(0.0);
        }
        // Set angular velocity when falling.
        if owner.get_linear_velocity().y > 0.0 {
            owner.set_angular_velocity(PI / 2.0);
        }
    }

    // Function connected with animation_finished() event in PuffAnimation child.
    #[export]
    unsafe fn _on_puff_animation_finished(&self, owner: RigidBody2D) {
      owner.get_node(NodePath::from_str("./PuffAnimation"))
            .and_then(|node| node.cast::<AnimatedSprite>())
            .expect("AnimationSprite doesnt't exist")
            .hide()

    }
}
