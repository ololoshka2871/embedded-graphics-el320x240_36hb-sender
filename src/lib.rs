mod el320x240_36hb_sender;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size};
use embedded_graphics_simulator::{OutputSettings, OutputSettingsBuilder, SimulatorDisplay};
use tokio::io::AsyncWriteExt;

static DISPLAY_SIZE: Size = Size::new(320, 240);

pub fn create_display() -> SimulatorDisplay<BinaryColor> {
    SimulatorDisplay::<BinaryColor>::new(DISPLAY_SIZE)
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
    mut port: Option<tokio_serial::SerialStream>,
    window: &mut embedded_graphics_simulator::Window,
    mut render: impl FnMut(&mut SimulatorDisplay<BinaryColor>) -> Result<(), E>,
) -> Result<(), E> {
    lazy_static::lazy_static! {
        static ref OUTPUT_SETTINGS: OutputSettings = OutputSettingsBuilder::new().scale(1).build();
    }

    loop {
        render(&mut display)?;

        window.update(&display);

        if window
            .events()
            .any(|e| e == embedded_graphics_simulator::SimulatorEvent::Quit)
        {
            if let Some(port) = port.as_mut() {
                println!("Flushing port... and exit");
                port.flush().await.unwrap();
            }
            break Ok(());
        }

        if let Some(port) = port.as_mut() {
            let img_binary = display
                .to_grayscale_output_image(&OUTPUT_SETTINGS)
                .as_image_buffer()
                .as_raw()
                .chunks(8)
                .map(|v| v.iter().fold(0, |acc, v| (acc << 1) | (*v != 0) as u8))
                .collect::<Vec<_>>();

            assert_eq!(
                img_binary.len(),
                (DISPLAY_SIZE.height * DISPLAY_SIZE.width / 8) as usize
            );

            if let Err(e) = el320x240_36hb_sender::send_to_display(port, &img_binary).await {
                panic!("Port error: {:?}", e);
            }
        }
    }
}
