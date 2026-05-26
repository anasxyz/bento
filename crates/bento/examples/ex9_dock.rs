use bento::*;

const DOCK_A_X: f32 = 50.0;
const DOCK_A_Y: f32 = 50.0;
const DOCK_B_X: f32 = 450.0;
const DOCK_B_Y: f32 = 50.0;
const DOCK_W: f32 = 300.0;
const DOCK_H: f32 = 300.0;
const SNAP_THRESHOLD: f32 = 200.0;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut dock_a = Group::new();
    dock_a.x = DOCK_A_X;
    dock_a.y = DOCK_A_Y;
    dock_a.width = Size::Fixed(DOCK_W);
    dock_a.height = Size::Fixed(DOCK_H);
    dock_a.background = Some([0.1, 0.2, 0.1, 1.0]);
    ui.add(dock_a);

    let mut dock_b = Group::new();
    dock_b.x = DOCK_B_X;
    dock_b.y = DOCK_B_Y;
    dock_b.width = Size::Fixed(DOCK_W);
    dock_b.height = Size::Fixed(DOCK_H);
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
    let btn1 = ui.add(Button::new("Button A"));
    let btn2 = ui.add(Button::new("Button B"));
    ui.append(panel, label);
    ui.append(panel, btn1);
    ui.append(panel, btn2);

    ui.listen(panel, move |_: &MouseUp, ui: &mut Ui| {
        ui.set(panel, |p| {
            let docks = [(DOCK_A_X, DOCK_A_Y), (DOCK_B_X, DOCK_B_Y)];
            for (dx, dy) in docks {
                if (p.x - dx).abs() < SNAP_THRESHOLD && (p.y - dy).abs() < SNAP_THRESHOLD {
                    p.x = dx;
                    p.y = dy;
                    break;
                }
            }
        });
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
