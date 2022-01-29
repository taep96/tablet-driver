use hidapi::HidApi;
use std::io::Write;

use crossterm::{cursor, execute, terminal};
use std::io::stdout;

// CTL-472
const VID: u16 = 0x056a;
const PID: u16 = 0x037a;

fn main() {
    // Connect to device
    let api = HidApi::new().expect("HidApi: Failed to create API instance");
    let tablet = api.open(VID, PID).expect("HidApi: Failed to open tablet");

    // Device info
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

    let mut stdout = stdout();

    // TODO: buffer length = report length (returned by tablet.read)
    let mut buf = [0u8; 10];

    loop {
        // Get data report
        tablet
            .read(&mut buf)
            .expect("HidApi: Failed to read to buffer");

        // Format
        let mut data_string = String::new();

        for u in &buf {
            data_string.push_str(&(u.to_string() + "\t"));
        }

        // Clear last iterations output
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToPreviousLine(1),
            terminal::Clear(terminal::ClearType::CurrentLine),
        )
        .expect("Crossterm: Failed to clear terminal");

        // Raw usb data
        println!("{}", data_string);

        // Parse pen coordinates
        let (mut pen_x, mut pen_y) = (
            i16::from_be_bytes([buf[3], buf[2]]) as f32,
            i16::from_be_bytes([buf[5], buf[4]]) as f32,
        );

        // Area position/offset
        pen_x -= 2000.0;
        pen_y -= 2000.0;

        // Remap to screen coordinates
        let (screen_x, screen_y) = (
            (pen_x / (15200.0 / 1920.0)) as i16,
            (pen_y / (9500.0 / 1080.0)) as i16,
        );

        // Cursor coordinates on screen
        print!("{}x{}\t", screen_x, screen_y);

        // Pen's coordinbates on tablet
        print!("{}x{}", pen_x, pen_y);

        // Flush to prevent lag
        stdout.flush().expect("Failed to flush");
    }
}
