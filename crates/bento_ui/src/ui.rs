use bento_wgpu::{DrawList, TextMeasurer};

use crate::View;

pub struct Ui {
    pub view: Box<dyn View>,
    pub measurer: TextMeasurer,
}

impl Ui {
    pub fn new(view: impl View + 'static) -> Self {
        Self {
            view: Box::new(view),
            measurer: TextMeasurer::new(),
        }
    }

    pub fn collect_draw_list(&mut self) -> DrawList {
        let mut draw_list = DrawList::new();
        self.view
            .render(0.0, 0.0, &mut self.measurer, &mut draw_list);
        draw_list
    }
}
