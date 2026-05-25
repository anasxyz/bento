#![allow(dead_code)]
#![allow(unused)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();
    ui.debug(true);

    let mut col = Group::new();
    col.layout = Layout::Column { gap: 4.0 };
    let col = ui.add(col);

    let mut handles = Vec::new();

    let texts = [
        "The quick brown fox jumps over the lazy dog",
        "Pack my box with five dozen liquor jugs",
        "How vexingly quick daft zebras jump",
        "The five boxing wizards jump quickly",
        "Sphinx of black quartz judge my vow",
        "Two driven jocks help fax my big quiz",
        "Five quacking zephyrs jolt my wax bed",
        "The jay pig fox zebra and my wolves",
        "Blowzy red vixens fight for a quick jump",
        "Jackdaws love my big sphinx of quartz",
        "Pack my red box with five dozen quality jugs",
        "Crazy Frederick bought many very exquisite opal jewels",
        "We promptly judged antique ivory buckles for the next prize",
        "A mad boxer shot a quick gloved jab to the jaw",
        "Jaded zombies acted quaintly but kept driving flux",
        "How quickly daft jumping zebras vex",
        "Bright vixens jump dozy fowl quack",
        "Quick wafting zephyrs vex bold jim",
        "Waltz nymph for quick jigs vex bud",
        "Glib jocks quiz nymph to vex dwarf",
        "Sphinx of black quartz hear my vow",
        "Pack my box with five dozen liquor",
        "The five boxing wizards jump fast",
        "How vexingly quick the daft zebras",
        "Jumpy halfback vows to protect mix",
    ];

    for (i, &text) in texts.iter().enumerate() {
        let mut t = Text::new(text);
        t.set_x(0.0);
        let h = ui.add(t);
        ui.append(col, h);
        handles.push(h);
    }

    ui.asyncs.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        move |ui: &mut Ui| {
            let new_texts = [
                "CHANGED: The quick brown fox jumps over the lazy dog again",
                "CHANGED: Pack my box with five dozen liquor jugs today",
                "CHANGED: How vexingly quick daft zebras jump around",
                "CHANGED: The five boxing wizards jump very quickly",
                "CHANGED: Sphinx of black quartz judge my vow now",
                "CHANGED: Two driven jocks help fax my big quiz today",
                "CHANGED: Five quacking zephyrs jolt my wax bed hard",
                "CHANGED: The jay pig fox zebra and my wolves howl",
                "CHANGED: Blowzy red vixens fight for a quick jump up",
                "CHANGED: Jackdaws love my big sphinx of quartz today",
                "CHANGED: Pack my red box with five dozen quality jugs here",
                "CHANGED: Crazy Frederick bought many very exquisite jewels",
                "CHANGED: We promptly judged antique ivory buckles for prize",
                "CHANGED: A mad boxer shot a quick gloved jab to jaw",
                "CHANGED: Jaded zombies acted quaintly but kept driving",
                "CHANGED: How quickly daft jumping zebras vex us all",
                "CHANGED: Bright vixens jump dozy fowl quack loudly",
                "CHANGED: Quick wafting zephyrs vex bold jim today",
                "CHANGED: Waltz nymph for quick jigs vex bud now",
                "CHANGED: Glib jocks quiz nymph to vex dwarf here",
            ];
            for (i, &text) in new_texts.iter().enumerate() {
                ui.set(handles[i], |w| w.set_content(text));
            }
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
