use gdnative::api::{CanvasLayer, Label};
use gdnative::prelude::{methods, NativeClass, TRef};

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
pub struct GameState {
    score: u32,
}

fn get_score_label(owner: TRef<CanvasLayer>) -> TRef<'_, Label> {
    owner
        .get_node("./ScoreLabel")
        .and_then(|cam| unsafe { cam.assume_safe().cast::<Label>() })
        .expect("There is no score label")
}

#[methods]
impl GameState {
    pub fn new(_owner: &CanvasLayer) -> Self {
        GameState { score: 0 }
    }

    #[export]
    fn handle_pass_pipe(&mut self, owner: TRef<CanvasLayer>) {
        self.score += 1;
        get_score_label(owner).set_text(self.score.to_string());
    }
}
