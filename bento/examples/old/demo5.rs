use bento::*;

fn build_login_ui() -> Ui {
    let mut ui = Ui::new();

    let root = ui.add(Container::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(8, 8, 8))
        .set_align_items(AlignItems::Center)
        .set_justify_content(JustifyContent::Center)
        .set_flex_direction(FlexDirection::Col);
    ui.set_root(root);

    let card = ui.add(Container::new());
    ui.get_mut(card)
        .unwrap()
        .set_width(Size::Fixed(380.0))
        .set_flex_direction(FlexDirection::Col)
        .set_color(Color::rgb(14, 14, 14))
        .set_radius(12.0)
        .set_border_color(Color::rgb(40, 10, 10))
        .set_border_widths([1.0; 4])
        .set_padding([36.0, 36.0, 36.0, 36.0])
        .set_row_gap(24.0);
    ui.append(root, card);

    let header = ui.add(Container::new());
    ui.get_mut(header)
        .unwrap()
        .set_flex_direction(FlexDirection::Col)
        .set_row_gap(6.0);
    ui.append(card, header);

    let title = ui.add(Label::new("Welcome back"));
    ui.get_mut(title)
        .unwrap()
        .set_size(22.0)
        .set_weight(600)
        .set_color(Color::rgb(240, 240, 240));
    ui.append(header, title);

    let subtitle = ui.add(Label::new("Sign in to your account"));
    ui.get_mut(subtitle)
        .unwrap()
        .set_size(13.0)
        .set_color(Color::rgb(100, 80, 80));
    ui.append(header, subtitle);

    let divider = ui.add(Container::new());
    ui.get_mut(divider)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Fixed(1.0))
        .set_color(Color::rgb(40, 10, 10));
    ui.append(card, divider);

    let fields = ui.add(Container::new());
    ui.get_mut(fields)
        .unwrap()
        .set_flex_direction(FlexDirection::Col)
        .set_row_gap(16.0);
    ui.append(card, fields);

    // email
    let email_group = ui.add(Container::new());
    ui.get_mut(email_group)
        .unwrap()
        .set_flex_direction(FlexDirection::Col)
        .set_row_gap(6.0);
    ui.append(fields, email_group);

    let email_label = ui.add(Label::new("Email address"));
    ui.get_mut(email_label)
        .unwrap()
        .set_size(12.0)
        .set_weight(500)
        .set_color(Color::rgb(160, 120, 120));
    ui.append(email_group, email_label);

    let email_input = ui.add(TextInput::new());
    ui.get_mut(email_input)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_placeholder("you@example.com")
        .set_font_size(20.0)
        .set_background_color(Color::rgb(20, 12, 12))
        .set_text_color(Color::rgb(225, 210, 210))
        .set_placeholder_color(Color::rgb(80, 50, 50))
        .set_border_color(Color::rgb(60, 20, 20))
        .set_border_width(1.0)
        .set_border_radius(7.0)
        .set_padding_x(12.0);
    ui.append(email_group, email_input);

    // password
    let pass_group = ui.add(Container::new());
    ui.get_mut(pass_group)
        .unwrap()
        .set_flex_direction(FlexDirection::Col)
        .set_row_gap(6.0);
    ui.append(fields, pass_group);

    let pass_label = ui.add(Label::new("Password"));
    ui.get_mut(pass_label)
        .unwrap()
        .set_size(12.0)
        .set_weight(500)
        .set_color(Color::rgb(160, 120, 120));
    ui.append(pass_group, pass_label);

    let pass_input = ui.add(TextInput::new());
    ui.get_mut(pass_input)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_placeholder("••••••••")
        .set_font_size(20.0)
        .set_background_color(Color::rgb(20, 12, 12))
        .set_text_color(Color::rgb(225, 210, 210))
        .set_placeholder_color(Color::rgb(80, 50, 50))
        .set_border_color(Color::rgb(60, 20, 20))
        .set_border_width(1.0)
        .set_border_radius(7.0)
        .set_padding_x(12.0);
    ui.append(pass_group, pass_input);

    // button
    let button = ui.add(Container::new());
    ui.get_mut(button)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Fixed(38.0))
        .set_color(Color::rgb(185, 28, 28))
        .set_radius(7.0)
        .set_align_items(AlignItems::Center)
        .set_justify_content(JustifyContent::Center);
    ui.append(card, button);

    let button_label = ui.add(Label::new("Sign in"));
    ui.get_mut(button_label)
        .unwrap()
        .set_size(14.0)
        .set_weight(600)
        .set_color(Color::rgb(255, 255, 255));
    ui.append(button, button_label);

    // footer
    let footer = ui.add(Container::new());
    ui.get_mut(footer)
        .unwrap()
        .set_justify_content(JustifyContent::Center)
        .set_align_items(AlignItems::Center)
        .set_col_gap(4.0);
    ui.append(card, footer);

    let footer_label = ui.add(Label::new("Don't have an account?"));
    ui.get_mut(footer_label)
        .unwrap()
        .set_size(12.0)
        .set_color(Color::rgb(90, 70, 70));
    ui.append(footer, footer_label);

    let signup_label = ui.add(Label::new("Sign up"));
    ui.get_mut(signup_label)
        .unwrap()
        .set_size(12.0)
        .set_weight(500)
        .set_color(Color::rgb(220, 60, 60));
    ui.append(footer, signup_label);

    ui
}

fn build_dashboard_ui() -> Ui {
    let mut ui = Ui::new();

    let root = ui.add(Container::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(8, 8, 8))
        .set_flex_direction(FlexDirection::Col);
    ui.set_root(root);

    // top bar
    let topbar = ui.add(Container::new());
    ui.get_mut(topbar)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Fixed(52.0))
        .set_color(Color::rgb(14, 14, 14))
        .set_border_color(Color::rgb(40, 10, 10))
        .set_border_widths([0.0, 0.0, 1.0, 0.0])
        .set_align_items(AlignItems::Center)
        .set_padding([0.0, 24.0, 0.0, 24.0])
        .set_flex_direction(FlexDirection::Row)
        .set_col_gap(12.0);
    ui.append(root, topbar);

    let app_name = ui.add(Label::new("BENTO"));
    ui.get_mut(app_name)
        .unwrap()
        .set_size(13.0)
        .set_weight(700)
        .set_color(Color::rgb(220, 60, 60));
    ui.append(topbar, app_name);

    let spacer = ui.add(Container::new());
    ui.get_mut(spacer).unwrap().set_flex_grow(1.0);
    ui.append(topbar, spacer);

    let user_label = ui.add(Label::new("Logged in"));
    ui.get_mut(user_label)
        .unwrap()
        .set_size(12.0)
        .set_color(Color::rgb(100, 80, 80));
    ui.append(topbar, user_label);

    // body
    let body = ui.add(Container::new());
    ui.get_mut(body)
        .unwrap()
        .set_flex_grow(1.0)
        .set_flex_direction(FlexDirection::Row);
    ui.append(root, body);

    // sidebar
    let sidebar = ui.add(Container::new());
    ui.get_mut(sidebar)
        .unwrap()
        .set_width(Size::Fixed(200.0))
        .set_height(Size::Percent(100.0))
        .set_color(Color::rgb(11, 11, 11))
        .set_border_color(Color::rgb(40, 10, 10))
        .set_border_widths([0.0, 1.0, 0.0, 0.0])
        .set_flex_direction(FlexDirection::Col)
        .set_padding([16.0, 16.0, 16.0, 16.0])
        .set_row_gap(4.0);
    ui.append(body, sidebar);

    for (label, active) in [
        ("Dashboard", true),
        ("Analytics", false),
        ("Users", false),
        ("Settings", false),
    ] {
        let item = ui.add(Container::new());
        ui.get_mut(item)
            .unwrap()
            .set_width(Size::Percent(100.0))
            .set_height(Size::Fixed(34.0))
            .set_color(if active {
                Color::rgb(40, 10, 10)
            } else {
                Color::TRANSPARENT
            })
            .set_radius(6.0)
            .set_align_items(AlignItems::Center)
            .set_padding([0.0, 0.0, 0.0, 12.0]);
        ui.append(sidebar, item);

        let item_label = ui.add(Label::new(label));
        ui.get_mut(item_label)
            .unwrap()
            .set_size(13.0)
            .set_weight(if active { 600 } else { 400 })
            .set_color(if active {
                Color::rgb(220, 60, 60)
            } else {
                Color::rgb(120, 100, 100)
            });
        ui.append(item, item_label);
    }

    // main content
    let content = ui.add(Container::new());
    ui.get_mut(content)
        .unwrap()
        .set_flex_grow(1.0)
        .set_flex_direction(FlexDirection::Col)
        .set_padding([28.0, 28.0, 28.0, 28.0])
        .set_row_gap(20.0);
    ui.append(body, content);

    let content_title = ui.add(Label::new("Dashboard"));
    ui.get_mut(content_title)
        .unwrap()
        .set_size(20.0)
        .set_weight(600)
        .set_color(Color::rgb(240, 240, 240));
    ui.append(content, content_title);

    // stat cards
    let stats_row = ui.add(Container::new());
    ui.get_mut(stats_row)
        .unwrap()
        .set_flex_direction(FlexDirection::Row)
        .set_col_gap(16.0);
    ui.append(content, stats_row);

    for (value, label) in [
        ("2,840", "Total Users"),
        ("128", "Active Now"),
        ("94%", "Uptime"),
    ] {
        let stat = ui.add(Container::new());
        ui.get_mut(stat)
            .unwrap()
            .set_width(Size::Fixed(160.0))
            .set_color(Color::rgb(14, 14, 14))
            .set_border_color(Color::rgb(40, 10, 10))
            .set_border_widths([1.0; 4])
            .set_radius(8.0)
            .set_padding([16.0, 16.0, 16.0, 16.0])
            .set_flex_direction(FlexDirection::Col)
            .set_row_gap(6.0);
        ui.append(stats_row, stat);

        let val = ui.add(Label::new(value));
        ui.get_mut(val)
            .unwrap()
            .set_size(24.0)
            .set_weight(700)
            .set_color(Color::rgb(220, 60, 60));
        ui.append(stat, val);

        let lbl = ui.add(Label::new(label));
        ui.get_mut(lbl)
            .unwrap()
            .set_size(12.0)
            .set_color(Color::rgb(100, 80, 80));
        ui.append(stat, lbl);
    }

    let section_title = ui.add(Label::new("Recent Activity"));
    ui.get_mut(section_title)
        .unwrap()
        .set_size(14.0)
        .set_weight(600)
        .set_color(Color::rgb(180, 150, 150));
    ui.append(content, section_title);

    // scroll list
    let scroll = ui.add(Container::new());
    ui.get_mut(scroll)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Fixed(220.0))
        .set_color(Color::rgb(14, 14, 14))
        .set_flex_direction(FlexDirection::Col);
    ui.append(content, scroll);

    for (i, event) in [
        "User alice@example.com signed in",
        "Password reset requested for bob@example.com",
        "New user registered: charlie@example.com",
        "Failed login attempt from 192.168.1.42",
        "User dave@example.com signed out",
        "Settings updated by admin",
        "Database backup completed",
        "User eve@example.com signed in",
        "API rate limit reached for key #4821",
        "System health check passed",
    ]
    .iter()
    .enumerate()
    {
        let row = ui.add(Container::new());
        ui.get_mut(row)
            .unwrap()
            .set_width(Size::Percent(100.0))
            .set_height(Size::Fixed(36.0))
            .set_align_items(AlignItems::Center)
            .set_padding([0.0, 16.0, 0.0, 16.0])
            .set_color(if i % 2 == 0 {
                Color::rgb(14, 14, 14)
            } else {
                Color::rgb(18, 10, 10)
            });
        ui.append(scroll, row);

        let entry = ui.add(Label::new(event));
        ui.get_mut(entry)
            .unwrap()
            .set_size(12.0)
            .set_color(Color::rgb(160, 130, 130));
        ui.append(row, entry);
    }

    ui
}

fn main() {
    let mut app = App::new();

    app.open_window(
        WindowConfig {
            title: "Sign in".to_string(),
            width: 600,
            height: 500,
            clear_color: Color::rgb(8, 8, 8),
        },
        build_login_ui(),
    );

    app.open_window(
        WindowConfig {
            title: "Dashboard".to_string(),
            width: 900,
            height: 600,
            clear_color: Color::rgb(8, 8, 8),
        },
        build_dashboard_ui(),
    );

    app.run();
}
