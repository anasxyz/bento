// --- ANIMATIONS ---
// derive Widget on your struct to get all of this automatically

// instant set - marks widget dirty, triggers redraw
widget.set_x(100.0);
widget.set_color([1.0, 0.0, 0.0, 1.0]);

// explicit animation - animate a field to a target value
// animate_<field>(to, duration_secs, easing, loop_mode)
widget.animate_x(500.0, 1.0, Easing::EaseInOut, LoopMode::Once);
widget.animate_color([1.0, 0.0, 0.0, 1.0], 0.5, Easing::Linear, LoopMode::Once);

// loop modes
LoopMode::Once      // plays once and stops
LoopMode::Loop      // restarts from beginning when complete
LoopMode::PingPong  // plays forward then backward, repeating

// easing
Easing::Linear
Easing::EaseIn
Easing::EaseOut
Easing::EaseInOut

// stop a looping animation
widget.stop_x_animation();
widget.stop_color_animation();

// transitions - any set_<field> call animates automatically
// set_transition_<field>(duration_secs, easing)
widget.set_transition_x(0.5, Easing::EaseInOut);
widget.set_x(200.0); // now animates instead of snapping

// default transition - applies to all fields without a specific transition
widget.base.default_transition = Some((0.5, Easing::EaseInOut));

// clear a transition - go back to instant set
widget.clear_transition_x();

// --- SUPPORTED FIELD TYPES ---
// f32         — animate_<field>, set_transition_<field>, clear_transition_<field>, stop_<field>_animation
// [f32; 4]    — same, with color interpolation
// everything else — set_<field> only, marks dirty

// --- UI HELPERS ---
// batch mutations without repeated get_mut calls
ui.with(handle, |w| {
    w.set_x(100.0);
    w.set_color([0.0, 1.0, 0.0, 1.0]);
    w.animate_opacity(0.0, 1.0, Easing::EaseOut, LoopMode::Once);
});

// --- CUSTOM WIDGETS ---
// implement build and update, everything else is generated
#[derive(Widget)]
pub struct MyWidget {
    pub base: Base,   // required
    pub x: f32,       // gets full animation API
    pub color: [f32; 4], // gets full animation API with color interpolation
    pub label: String,   // gets set_label only
    id: Option<SceneNodeId>, // private, no setter generated
}

impl Widget for MyWidget {
    fn build(&mut self, scene: &mut Scene) { ... }
    fn update(&mut self, scene: &mut Scene, measurer: &mut dyn TextMeasurer) { ... }
}
