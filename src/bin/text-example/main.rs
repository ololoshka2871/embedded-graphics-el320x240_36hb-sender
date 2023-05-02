use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};

fn render<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let large_text = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let centered = TextStyleBuilder::new()
        .baseline(Baseline::Middle)
        .alignment(Alignment::Center)
        .build();

    Text::with_text_style(
        "embedded-graphics",
        display.bounding_box().center(),
        large_text,
        centered,
    )
    .draw(display)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut display = el320x240_36hb_sender::create_display();
    let mut window = el320x240_36hb_sender::create_window("Test");

    el320x240_36hb_sender::crate_render_loop(&mut display, &mut window, render)
        .await
        .unwrap();
}
