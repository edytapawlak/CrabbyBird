use gdnative::{godot_error, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods};
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
        // Pipe passed
        self.score += 1;
        if let Some(mut score_lbl) = self.score_label {
            unsafe {
                score_lbl.set_text(GodotString::from_str(self.score.to_string()));
            }
        }
    }
}
