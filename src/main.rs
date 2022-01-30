use hidapi::HidApi;

use crossterm::{cursor, execute, terminal};
use fprint::fprint;

// CTL-472
const VID: u16 = 0x056a;
const PID: u16 = 0x037a;

fn main() {
    let api = HidApi::new().expect("HidApi: Failed to create API instance");
    let tablet = api.open(VID, PID).expect("HidApi: Failed to open tablet");

    println!(
        "\
Successfully connected to:
  Manufacturer: {}
  Product: {}
  Serial Number: {}

",
        tablet
            .get_manufacturer_string()
            .ok()
            .flatten()
            .expect("HidApi: Failed to get manufacturer string"),
        tablet
            .get_product_string()
            .ok()
            .flatten()
            .expect("HidApi: Failed to get product string"),
        tablet
            .get_serial_number_string()
            .ok()
            .flatten()
            .expect("HidApi: Failed to get serial number"),
    );

    let mut stdout = std::io::stdout();

    // TODO: buffer length = report length (returned by tablet.read)
    let mut buf = [0u8; 10];

    loop {
        tablet
            .read(&mut buf)
            .expect("HidApi: Failed to read to buffer");

        // Make more readable | 255    255    ...
        let mut data_string = String::new();

        for u in &buf {
            data_string.push_str(&(u.to_string() + "\t"));
        }

        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToPreviousLine(1),
            terminal::Clear(terminal::ClearType::CurrentLine),
        )
        .expect("Crossterm: Failed to clear terminal");

        // Parse pen coordinates
        let (pen_x, pen_y) = (
            u16::from_be_bytes([buf[3], buf[2]]) as f32,
            u16::from_be_bytes([buf[5], buf[4]]) as f32,
        );

        // Convert pen coordinates
        let (screen_x, screen_y) = (
            ((pen_x - 2000.0) // - x_offset
            / (15200.0 / 1920.0)) as u16,
            ((pen_y - 2000.0) // - y_offset
            / (9500.0 / 1080.0)) as u16,
        );

        println!("{}", data_string);

        fprint!("{}x{}\t", screen_x, screen_y);
        fprint!("{}x{}", pen_x, pen_y);
    }
}
