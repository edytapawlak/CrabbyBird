use gdnative::{
    godot_error, godot_print, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods,
};
use gdnative::{CanvasLayer, GodotString, Label, NativeClass, Node2D, NodePath, Vector2, };
use std::cmp;
use gdnative::init::{ClassBuilder, Signal};

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
#[register_with(Self::register_signals)]
pub struct GameState {
    score_label: Option<Label>,
    score: u32,
    game_over_node: Option<Node2D>,
    summary_label: Option<Label>,
    best_score: u32,
    best_score_label: Option<Label>,
}

#[methods]
impl GameState {
    pub fn _init(_owner: CanvasLayer) -> Self {
        GameState {
            score_label: None,
            score: 0,
            game_over_node: None,
            best_score: 0,
            best_score_label: None,
            summary_label: None,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: CanvasLayer) {
        self.score_label = owner
            .get_node(NodePath::from_str("./ScoreLabel"))
            .and_then(|n| n.cast::<Label>());
        self.game_over_node = owner
            .get_node(NodePath::from_str("./GameOverNode"))
            .and_then(|n| n.cast::<Node2D>());

        if let Some(mut game_over_node) = self.game_over_node {
            self.best_score_label = game_over_node
                .get_node(NodePath::from_str("./BestScoreLabel"))
                .and_then(|n| n.cast::<Label>());
            self.summary_label = game_over_node
                .get_node(NodePath::from_str("./SummaryLabel"))
                .and_then(|n| n.cast::<Label>());
            game_over_node.set_visible(false);

            let screen_size = owner
              .get_viewport()
              .expect("Can't get screen size.")
              .get_size();
            game_over_node.set_position(Vector2::new(screen_size.x/2., screen_size.y/3.));
        } else {
            godot_print!("There is no game_over_node.");
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
      builder.add_signal(Signal {
          name: "new_game_pressed",
          args: &[],
      });
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

    #[export]
    fn game_over(&mut self, mut _owner: gdnative::CanvasLayer) {
        godot_print!("Game Over!");
        // Update best score.
        self.best_score = cmp::max(self.score, self.best_score);

        unsafe {
            self.best_score_label.and_then(|mut label| {
                Some(label.set_text(GodotString::from_str(self.best_score.to_string())))
            });
            self.summary_label.and_then(|mut label| {
                Some(label.set_text(GodotString::from_str(self.score.to_string())))
            });
            self.game_over_node
                .and_then(|mut game_over_n| Some(game_over_n.set_visible(true)));
            self.score_label
                .and_then(|mut score_lab| Some(score_lab.set_visible(false)));
        }
    }

    #[export]
    unsafe fn _on_new_game_button_pressed(&mut self, mut owner: gdnative::CanvasLayer) {
      self.score = 0;
      owner.emit_signal(GodotString::from_str("new_game_pressed"), &[]);
    }
    
}
