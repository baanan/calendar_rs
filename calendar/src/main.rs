use canvas_tui::prelude::*;

fn main() {
    let mut canvas = Basic::filled_with_text(&(9, 5), '.')
        .when_error(|canvas, _| {
            canvas.fill('x')?;
            Ok(())
        });

    canvas
        .rect_absolute(&(1, 1), &(7, 3), &box_chars::LIGHT)
        .text(&Just::Centered, "foo")
        .discard_result();

    let _ = canvas.print_monochrome();
}
