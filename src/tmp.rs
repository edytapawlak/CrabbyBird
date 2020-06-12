use gdnative::*;

enum State {
    Loading,
    Running,
    GameOver,
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GameScene {
    state: State,
    player: Option<RigidBody2D>,
    obstacle_obj: Option<PackedScene>,
}

#[methods]
impl GameScene {
    pub fn _init(_owner: Node2D) -> Self {
        godot_print!("GameScene!");
        GameScene {
            state: State::Loading,
            player: None,
            obstacle_obj: None,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, mut owner: gdnative::Node2D) {
        let mut resource_loader = ResourceLoader::godot_singleton();
        
        // Setting obstale object
        self.obstacle_obj = resource_loader
            .load("res://scenes/Pipe.tscn".into(), "".into(), false)
            .and_then(|res| res.cast::<PackedScene>());

        self.spawn_obstacles(owner);
    }

    // Create obstacle, set its position to x, y and add it as a child of game_scene.
    unsafe fn spawn_obstacle(&mut self, mut owner: Node2D, x: f32, y: f32) {
        if let Some(obstacle_obj) = self.obstacle_obj.take() {
            let mut obstacle = obstacle_obj
                .instance(0)
                .and_then(|node| node.cast::<StaticBody2D>())
                .expect("Could not create obstacle instance!");
            obstacle.set_position(Vector2::new(x, y));
            owner.add_child(Some(obstacle.to_node()), false);
            self.obstacle_obj.replace(obstacle_obj);
        }
    }

    // Spawn ten obstacles, that is temporary
    unsafe fn spawn_obstacles(&mut self, owner: Node2D) {
        let mut generator = RandomNumberGenerator::new();
        generator.randomize();

        for i in 0..=10 {
            let bottom = owner.get_viewport_rect().size.height;
            self.spawn_obstacle(owner, (10 * i) as f32, generator.randi() as f32 % bottom);
        }
    }
}


//--------------------------------------
//use gdnative::*;

//#[derive(NativeClass)]
//#[inherit(StaticBody2D)]
//pub struct Obstacle;

//#[methods]
//impl Obstacle {
//    pub fn _init(_owner: StaticBody2D) -> Self {
//        Obstacle
//    }
//  }
