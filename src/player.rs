use gdnative::*;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    standard_gravity: Vector2,
    jump_speed: f32,
    velocity: Vector2,
}

#[methods]
impl Player {
    pub fn _init(_owner: KinematicBody2D) -> Self {
        godot_print!("player!");

        Player {
            standard_gravity: Vector2::new(0.0, 800.0),
            jump_speed: 800.0,
            velocity: Vector2::new(0., 0.),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, mut owner: KinematicBody2D) {
        let size = owner.get_viewport_rect().size;
        owner.set_position(Vector2::new(size.width / 2., size.height / 2.));
        owner.set_physics_process(true)
    }

    fn get_input(&mut self) {
        let input = Input::godot_singleton();
        if Input::is_action_just_pressed(&input, GodotString::from_str("ui_select")) {
            self.velocity += Vector2::new(0.0, -self.jump_speed);
        }
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut owner: KinematicBody2D, delta: f64) {
        self.get_input();
        self.velocity += Vector2::new(0.0, (delta as f32) * self.standard_gravity.y);
        owner.move_and_slide(self.velocity, Vector2::zero(), false, 4, 0.7, true);
    }
}
