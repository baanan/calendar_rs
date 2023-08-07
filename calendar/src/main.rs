use canvas_tui::prelude::*;

fn main() {
    if let Err(err) = draw() { println!("{err}") }
}

fn draw() -> Result<(), Error> {
    use themes::catppuccin::Frappe as Theme;
    // use themes::OneDark;

    #[allow(unused_variables)]
    let rosewater = rgb(242, 213, 207);
    #[allow(unused_variables)]
    let highlight_text = rgb(48, 52, 70);

    let widgets = widgets::Selectable::num(Theme.highlighted(Theme::rosewater()), 4, true);
    let mut canvas = Basic::filled_with(&(15, 12), ' ', Theme::text(), Theme::base());
    // let widgets = widgets::Selectable::num(OneDark.highlighted(OneDark::cyan()), 4, true);
    // let mut canvas = Basic::filled_with(&(15, 12), ' ', OneDark::text(), OneDark::base());

    let width = "Macchiato".len() + 2;
    #[allow(clippy::cast_possible_wrap)]
    canvas
        .draw(&Just::CenteredOnRow(1),  widgets.title("Preferences"))
        .draw(&Just::CenteredOnRow(3),  widgets.titled_text(0.., "Theme", &["Latte", "Frappe", "Macchiato", "Mocha"]))
        .draw(&Just::CenteredOnRow(9),  widgets.rolling_selection(4, "abcdefghijklmnopqrstuvwxyz", width)
                                               .highlighted(Theme::blue()).build().at_start(true))
        .draw(&Just::CenteredOnRow(10), widgets.toggle(&5, "bello", true).width(width))?;

    canvas.print()?;

    Ok(())
}
