use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size};
use embedded_graphics_simulator::SimulatorDisplay;

pub fn create_display() -> SimulatorDisplay<BinaryColor> {
    SimulatorDisplay::<BinaryColor>::new(Size::new(320, 240))
}

pub fn create_window(title: &str) -> embedded_graphics_simulator::Window {
    let output_settings = embedded_graphics_simulator::OutputSettingsBuilder::new()
        .scale(1)
        .max_fps(30)
        .build();

    embedded_graphics_simulator::Window::new(title, &output_settings)
}

pub async fn crate_render_loop<E>(
    mut display: &mut SimulatorDisplay<BinaryColor>,
    window: &mut embedded_graphics_simulator::Window,
    mut render: impl FnMut(&mut SimulatorDisplay<BinaryColor>) -> Result<(), E>,
) -> Result<(), E> {
    loop {
        render(&mut display)?;

        window.update(&display);

        if window
            .events()
            .any(|e| e == embedded_graphics_simulator::SimulatorEvent::Quit)
        {
            break Ok(());
        }

        tokio::time::sleep(tokio::time::Duration::from_micros(20)).await;
    }
}
