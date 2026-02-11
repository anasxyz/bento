use rentex::widgets::{ButtonWidget, SliderWidget, TextInputWidget, WidgetHandle};
use rentex::{App, Ctx, FontId, Fonts, RentexApp};

struct Demo {
    name_input:    Option<WidgetHandle<TextInputWidget>>,
    message_input: Option<WidgetHandle<TextInputWidget>>,
    send_btn:      Option<WidgetHandle<ButtonWidget>>,
    clear_btn:     Option<WidgetHandle<ButtonWidget>>,
    size_slider:   Option<WidgetHandle<SliderWidget>>,
    font_small:    Option<FontId>,
    last_msg_len:  usize,
}

impl RentexApp for Demo {
    fn setup(&mut self, ctx: &mut Ctx) {
        let font = ctx.fonts.add("JetBrainsMono Nerd Font", 14.0);
        self.font_small = Some(font);

        let name_input = ctx.widgets.text_input();
        ctx.widgets.get_mut(name_input)
            .font(font)
            .size(280.0, 32.0)
            .placeholder("name...");
        self.name_input = Some(name_input);

        let message_input = ctx.widgets.text_input();
        ctx.widgets.get_mut(message_input)
            .font(font)
            .size(280.0, 32.0)
            .placeholder("message...");
        self.message_input = Some(message_input);

        let send_btn = ctx.widgets.button("send");
        ctx.widgets.get_mut(send_btn)
            .font(font)
            .size(84.0, 32.0)
            .color([0.18, 0.48, 0.92, 1.0])
            .hover_color([0.25, 0.58, 1.0, 1.0])
            .press_color([0.12, 0.35, 0.75, 1.0]);
        self.send_btn = Some(send_btn);

        let clear_btn = ctx.widgets.button("clear");
        ctx.widgets.get_mut(clear_btn)
            .font(font)
            .size(84.0, 32.0)
            .color([0.30, 0.12, 0.12, 1.0])
            .hover_color([0.48, 0.18, 0.18, 1.0])
            .press_color([0.20, 0.08, 0.08, 1.0]);
        self.clear_btn = Some(clear_btn);

        let size_slider = ctx.widgets.slider();
        ctx.widgets.get_mut(size_slider)
            .size(280.0, 18.0)
            .range(0.0, 100.0)
            .value(50.0)
            .show_label(font);
        self.size_slider = Some(size_slider);

        let btn_row = ctx.layout
            .hstack()
            .gap(8.0)
            .add(send_btn)
            .add(clear_btn)
            .as_ref();

        ctx.layout
            .vstack()
            .position(24.0, 24.0)
            .gap(10.0)
            .add(name_input)
            .add(message_input)
            .add(size_slider)
            .add_container(btn_row);
    }

    fn update(&mut self, ctx: &mut Ctx) {
        let send_btn      = self.send_btn.unwrap();
        let clear_btn     = self.clear_btn.unwrap();
        let name_input    = self.name_input.unwrap();
        let message_input = self.message_input.unwrap();

        if ctx.widgets.get(send_btn).just_clicked {
            let name = ctx.widgets.get(name_input).value.clone();
            let msg  = ctx.widgets.get(message_input).value.clone();
            let name = if name.trim().is_empty() { "anon".into() } else { name };
            let _msg = if msg.trim().is_empty()  { "(empty)".into() } else { msg };
            ctx.widgets.get_mut(message_input).value("");
        }

        if ctx.widgets.get(clear_btn).just_clicked {
            ctx.widgets.get_mut(name_input).value("");
            ctx.widgets.get_mut(message_input).value("");
        }

        let msg_len = ctx.widgets.get(message_input).value.len();
        if msg_len != self.last_msg_len {
            self.last_msg_len = msg_len;
            let label = if msg_len > 0 {
                format!("send ({})", msg_len)
            } else {
                "send".to_string()
            };
            ctx.widgets.get_mut(send_btn).text(label);
        }
    }
}

fn main() {
    App::new("rntx demo", 560, 480).run(
        Fonts::new(),
        Demo {
            name_input:    None,
            message_input: None,
            send_btn:      None,
            clear_btn:     None,
            size_slider:   None,
            font_small:    None,
            last_msg_len:  0,
        },
    );
}
