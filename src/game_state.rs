use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{CanvasLayer, GodotString, Label, NativeClass, NodePath};

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
pub struct GameState {
    score_label: Option<Label>,
    score: u32,
}

#[methods]
impl GameState {
    pub fn _init(_owner: CanvasLayer) -> Self {
        GameState {
            score_label: None,
            score: 0,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: CanvasLayer) {
        self.score_label = owner
            .get_node(NodePath::from_str("./ScoreLabel"))
            .and_then(|n| n.cast::<Label>());
    }

    #[export]
    fn notify_pass_pipe(&mut self, mut _owner: gdnative::CanvasLayer) {
        godot_print!("PIPE PASSED!");
        self.score += 1;
        unsafe {
            self.score_label
                .unwrap()
                .set_text(GodotString::from_str(format!("{:?}", self.score)));
        }
    }
}
