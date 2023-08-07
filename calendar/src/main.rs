use canvas_tui::{prelude::*, themes::{BasicTheme, OneDark}};

fn main() -> Result<(), Error> {
    // let mut canvas = Basic::filled_with_text(&(11, 5), '.')
    //     .when_error(|canvas, _| {
    //         canvas.fill('x')?;
    //         Ok(())
    //     });

    // canvas
    //     .rect(&Just::Centered, &(9, 3), &box_chars::LIGHT)
    //         .fill_inside(' ')
    //     .text(&Just::Centered, "foo")
    //         .grow_profile(&(1, 0))
    //         .colored(highlight_text, rosewater)
    //     .discard_result();

    // let mut canvas = Basic::filled_with_text(&(9, 7), '·')
    //     .when_error(|canvas, _| {
    //         canvas.fill('x')?;
    //         Ok(())
    //     });

    // canvas
    //     .grid(&Just::Centered, &(2, 1), &(2, 2), &box_chars::LIGHT)
    //         .draw_inside(Box::new(|mut canvas, cell| {
    //             canvas.text(&Just::Centered, &format!("{}{}", cell.x, cell.y))?; 
    //             Ok(())
    //         }))
    //         .inside().grow_profile(&(1, 0)).colored(HIGHLIGHT_TEXT, ROSEWATER)
    //     .discard_result();
    
    // struct Frappe;

    // impl Frappe {
    //     pub const fn rosewater() -> Color { Color::new(242, 213, 207) }
    //     pub const fn base() -> Color { Color::new(48, 52, 70) }
    // }

    // impl Theme for Frappe {
    //     fn title_fg(&self) -> Color { Self::base() }
    //     fn title_bg(&self) -> Color { Self::rosewater() }
    // }

    // canvas_tui::init();

    // let widgets = Themed::new(Frappe);

    // let mut canvas = Basic::filled_with_text(&(7, 3), '·')
    //     .when_error(|canvas, _| {
    //         canvas.fill('x')?;
    //         Ok(())
    //     });

    // canvas.draw(&Just::Centered, widgets.title("foo"))
    //     .discard_result();

    // use widgets::basic;
    // use themes::catppuccin::Frappe;

    // let mut canvas = Basic::new(&(9, 3));
    // canvas.draw(&Just::Centered, basic::toggle("foo", false, Frappe::base(), Frappe::rosewater()))?;
    
    // // ·········
    // // ·-foo-✕-· (highlight represented by -)
    // // ·········
    // assert_eq!(canvas.get(&(1, 1))?.foreground, Some(Frappe::base()));
    // assert_eq!(canvas.get(&(1, 1))?.background, Some(Frappe::rosewater()));
    // assert_eq!(canvas.get(&(6, 1))?.text, '✕');

    // let _ = canvas.print();

    // use widgets::selectable::Selectable;
    // use themes::catppuccin::Frappe;

    // let widgets = Selectable::num(Frappe, 1);
    // let mut canvas = Basic::new(&(9, 3));
    // canvas.draw(&Just::Centered, basic::toggle("foo", false, Frappe::base(), Frappe::rosewater()))?;
    
    // // ·········
    // // ·-foo-✕-· (highlight represented by -)
    // // ·········
    // assert_eq!(canvas.get(&(1, 1))?.foreground, Some(Frappe::base()));
    // assert_eq!(canvas.get(&(1, 1))?.background, Some(Frappe::rosewater()));
    // assert_eq!(canvas.get(&(6, 1))?.text, '✕');

    // let _ = canvas.print();

    // use themes::catppuccin::Frappe as Theme;

    #[allow(unused_variables)]
    let rosewater = Color::new(242, 213, 207);
    #[allow(unused_variables)]
    let highlight_text = Color::new(48, 52, 70);

    // let widgets = widgets::Selectable::num(Theme.highlighted(Theme::rosewater()), 4, true);
    // let mut canvas = Basic::filled_with(&(15, 11), ' ', Theme::text(), Theme::base());
    let widgets = widgets::Selectable::num(OneDark.highlighted(OneDark::cyan()), 4, true);
    let mut canvas = Basic::filled_with(&(15, 11), ' ', OneDark::text(), OneDark::base());

    #[allow(clippy::cast_possible_wrap)]
    canvas
        .draw(&Just::CenteredOnRow(1), widgets.title("Preferences"))
        .draw(&Just::CenteredOnRow(3), widgets.titled_text(0.., "Theme", &["Latte", "Frappe", "Macchiato", "Mocha"]))
        .draw(&Just::CenteredOnRow(9), widgets.rolling_selection(4, "abcdefghijklmnopqrstuvwxyz", "Macchiato".len() + 2))?;

    canvas.print()?;

    Ok(())
}
