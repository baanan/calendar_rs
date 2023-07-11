use canvas_tui::prelude::*;

fn main() {
    canvas_tui::init();

    let rosewater = Color::new(242, 213, 207);
    let highlight_text = Color::new(48, 52, 70);

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

    let mut canvas = Basic::filled_with_text(&(9, 7), '.')
        .when_error(|canvas, _| {
            canvas.fill('x')?;
            Ok(())
        });

    canvas
        .grid(&Just::Centered, &(2, 1), &(2, 2), &box_chars::LIGHT)
            .draw_inside(Box::new(|mut canvas, cell| {
                canvas.text(&Just::Centered, &format!("{}{}", cell.x, cell.y))?; 
                Ok(())
            }))
            .inside().grow_profile(&(1, 0)).colored(highlight_text, rosewater)
        .discard_result();

    let _ = canvas.print();
}
