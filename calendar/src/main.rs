use canvas_tui::prelude::*;

fn main() {
    canvas_tui::init();

    let rosewater = Color::new(242, 213, 207);
    let highlight_text = Color::new(48, 52, 70);

    let mut canvas = Basic::filled_with_text(&(9, 5), '.')
        .when_error(|canvas, _| {
            canvas.fill('x')?;
            Ok(())
        });

    canvas
        .rect_absolute(&(1, 1), &(7, 3), &box_chars::LIGHT)
            // .inside() // alternative of shrink(&(1, 1))
            // .filled_with(' ')
        .text(&Just::Centered, "foo")
            .grow_bounds(&(1, 0))
            .colored(highlight_text, rosewater)
        .discard_result();

    let _ = canvas.print();
}
