use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let status = ui.add(Text::new("Press Fetch to get a joke...", 20.0, 20.0, 18.0));
    let btn = ui.add(Button::new("Fetch", 20.0, 60.0, 120.0, 40.0));

    ui.listen(btn, move |_e: &Click, ui| {
        if let Some(t) = ui.get_mut(status) {
            t.set_text("Loading...");
        }

        ui.asyncs.spawn(async move {
            let result = async {
                let body = reqwest::get("https://icanhazdadjoke.com/slack")
                    .await?
                    .text()
                    .await?;
                let v: serde_json::Value = serde_json::from_str(&body)?;
                let joke = v["attachments"][0]["text"]
                    .as_str()
                    .unwrap_or("No joke found.")
                    .to_string();
                Ok::<String, Box<dyn std::error::Error + Send + Sync>>(joke)
            }
            .await
            .unwrap_or_else(|_| "Failed to fetch joke.".to_string());

            move |ui: &mut Ui| {
                if let Some(t) = ui.get_mut(status) {
                    t.set_text(&result);
                }
            }
        });
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
