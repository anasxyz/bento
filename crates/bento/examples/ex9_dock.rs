use bento::*;

const DOCK_A_X: f32 = 50.0;
const DOCK_A_Y: f32 = 50.0;
const DOCK_B_X: f32 = 450.0;
const DOCK_B_Y: f32 = 50.0;
const DOCK_A_W: f32 = 300.0;
const DOCK_A_H: f32 = 300.0;
const DOCK_B_W: f32 = 200.0;
const DOCK_B_H: f32 = 100.0;
const PANEL_W: f32 = 300.0;
const PANEL_H: f32 = 300.0;
const SNAP_THRESHOLD: f32 = 150.0;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut dock_a = Group::new();
    dock_a.x = DOCK_A_X;
    dock_a.y = DOCK_A_Y;
    dock_a.width = Size::Fixed(DOCK_A_W);
    dock_a.height = Size::Fixed(DOCK_A_H);
    dock_a.background = Some([0.1, 0.2, 0.1, 1.0]);
    ui.add(dock_a);

    let mut dock_b = Group::new();
    dock_b.x = DOCK_B_X;
    dock_b.y = DOCK_B_Y;
    dock_b.width = Size::Fixed(DOCK_B_W);
    dock_b.height = Size::Fixed(DOCK_B_H);
    dock_b.background = Some([0.1, 0.1, 0.2, 1.0]);
    ui.add(dock_b);

    let mut panel = Group::new();
    panel.layout = Layout::Column { gap: 8.0 };
    panel.x = 500.0;
    panel.y = 400.0;
    panel.draggable = true;
    panel.scrollable = true;
    panel.width = Size::Fixed(300.0);
    panel.height = Size::Fixed(300.0);
    panel.background = Some([0.15, 0.15, 0.15, 1.0]);
    panel.clip = true;
    let panel = ui.add(panel);

    let label = ui.add(Text::new("Drag me"));
    ui.append(panel, label);

    let minput = ui.add(MultilineInput::new());
    ui.append(panel, minput);
    ui.set(minput, |e| {
        e.set_font_size(12.0);
        e.color = [0.88, 0.88, 0.88, 1.0];
        e.width = Size::Fill;
        e.height = Size::Fill;
        e.use_spaces = true;
    });

    ui.listen(panel, move |_: &MouseUp, ui: &mut Ui| {
        ui.set(panel, |p| {
            let docks = [
                (DOCK_A_X, DOCK_A_Y, DOCK_A_W, DOCK_A_H),
                (DOCK_B_X, DOCK_B_Y, DOCK_B_W, DOCK_B_H),
            ];
            let mut snapped = false;
            for (dx, dy, dw, dh) in docks {
                if (p.x - dx).abs() < SNAP_THRESHOLD && (p.y - dy).abs() < SNAP_THRESHOLD {
                    p.x = dx;
                    p.y = dy;
                    p.width = Size::Fixed(dw);
                    p.height = Size::Fixed(dh);
                    snapped = true;
                    break;
                }
            }
            if !snapped {
                p.width = Size::Fixed(PANEL_W);
                p.height = Size::Fixed(PANEL_H);
            }
        });
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
