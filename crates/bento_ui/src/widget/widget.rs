use crate::ui::Ui;

pub trait Widget {
    fn build(&mut self, ui: &mut Ui);
    fn update(&mut self, ui: &mut Ui);
}
