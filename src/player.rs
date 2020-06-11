use gdnative::*;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Player {
    jump_speed: f32,
    facing_angle: f32, // maximal angle bird can faces up, in degrees
}

#[methods]
impl Player {
    pub fn _init(mut _owner: RigidBody2D) -> Self {
        Player {
            jump_speed: 400.0,
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
        // Modifing only y coordinates of players_velocity
        owner.set_linear_velocity(Vector2::new(
            owner.get_linear_velocity().x,
            -self.jump_speed,
        ));
        // Make player rotate clockwise while jumping
        owner.set_angular_velocity(-3.0);
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
        let facing_angle = self.facing_angle as f64;
        if actual_rotation < facing_angle && actual_rotation > -180.0 {
            owner.set_rotation_degrees(facing_angle);
            owner.set_angular_velocity(0.0);
        }
        // Setting angular velocity while player's falling
        if owner.get_linear_velocity().y > 0.0 {
            owner.set_angular_velocity(1.5);
        }
    }
}
