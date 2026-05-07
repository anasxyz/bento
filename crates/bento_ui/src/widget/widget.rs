use bento_shared::scene::Scene;

pub trait Widget {
    fn build(&mut self, scene: &mut Scene);
    fn update(&mut self, scene: &mut Scene);
}
