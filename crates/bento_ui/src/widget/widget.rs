use bento_shared::{TextMeasurer, scene::Scene};

pub trait Widget {
    fn build(&mut self, scene: &mut Scene);
    fn update(&mut self, scene: &mut Scene, _measurer: &mut dyn TextMeasurer);
}
