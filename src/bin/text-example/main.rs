mod render;

use std::time::Duration;

use clap::Parser;

use tokio_serial::SerialPortBuilderExt;

/// Sender example
#[derive(Parser)]
#[allow(non_snake_case)]
struct Cli {
    /// List avaliable serial ports
    #[clap(short, long, default_value_t = false)]
    list: bool,
    /// Test only, do not send data to display
    #[clap(short, long, default_value_t = false)]
    test: bool,
    /// Serial port to connect to
    port: Option<String>,
}

#[tokio::main]
async fn main() {
    use el320x240_36hb_sender::*;

    let args = Cli::parse();

    let ser_stream = if args.list {
        println!("Avaliable serial ports:");
        for port in tokio_serial::available_ports().unwrap() {
            println!("{}", port.port_name);
        }
        return;
    } else if args.test {
        println!("Test mode");
        None
    } else {
        let p = args.port.unwrap();
        println!("Connecting to '{}'", p);

        Some(
            tokio_serial::new(p, 1_500_000)
                .timeout(Duration::from_millis(100))
                .open_native_async()
                .unwrap(),
        )
    };

    let mut display = create_display();
    let mut window = create_window("Test");

    crate_render_loop(&mut display, ser_stream, &mut window, render::render)
        .await
        .unwrap();
}
