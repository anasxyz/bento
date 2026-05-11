use crate::Ui;

pub trait Widget {
    fn name(&self) -> &str;
    fn build(&mut self);
    fn update(&mut self);
}
