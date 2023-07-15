use canvas_tui::{prelude::*, shapes::Rect};

const ROSEWATER: Color = Color::new(242, 213, 207);
const HIGHLIGHT_TEXT: Color = Color::new(48, 52, 70);

fn main() {
    canvas_tui::init();


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

    let widgets = Widgets;

    let mut canvas = Basic::filled_with_text(&(9, 5), '·')
        .when_error(|canvas, _| {
            canvas.fill('x')?;
            Ok(())
        });

    canvas.draw(widgets.title(&Just::CenteredOnRow(1), "bello"))
        .discard_result();

    let _ = canvas.print();
}

struct Widgets;

impl Widgets {
    pub fn title<'a, C: Canvas>(&'a self, just: &'a Just, text: &'a str) 
        -> impl FnOnce(&mut C) -> DrawResult<C::Output, Rect> + '_ 
    {
        move |canvas| {
            canvas.text(just, text)
                .grow_profile(&(1, 0))
                .colored(HIGHLIGHT_TEXT, ROSEWATER)
        }
    }
}
