use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::fmt::Debug;

fn render<D>(display: &mut D)
where
    D: DrawTarget<Color = BinaryColor>,
    <D as embedded_graphics::draw_target::DrawTarget>::Error: Debug,
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
    .draw(display)
    .unwrap();
}

fn main() {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new().scale(1).max_fps(30).build();

    let mut window = Window::new("Themes", &output_settings);

    loop {
        render(&mut display);

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break;
        }
    }
}
