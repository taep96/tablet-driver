use hidapi::HidApi;
use xrandr::XHandle;

use crossterm::{cursor, execute, terminal};
use fprint::fprint;

// CTL-472
const VID: u16 = 0x056a;
const PID: u16 = 0x037a;

const TABLET_WIDTH: f32 = 15200.0;
const TABLET_HEIGHT: f32 = 9500.0;

fn main() {
    let api = HidApi::new().expect("HidApi: Failed to create API instance");
    let tablet = api.open(VID, PID).expect("HidApi: Failed to open tablet");

    println!("Successfully connected to:\n");
    println!(
        "  Tablet:
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

    // TODO: All monitors
    let monitor = &XHandle::open().unwrap().monitors().unwrap()[0];
    println!(
        "  Monitor(s):
    Name: {}
    Is primary?: {}
    Is automatic?: {}
    Start: {}x{}px
    Size px: {}x{}px
    Size mm: {}x{}mm",
        monitor.name,
        monitor.is_primary,
        monitor.is_automatic,
        monitor.x,
        monitor.y,
        monitor.width_px,
        monitor.height_px,
        monitor.width_mm,
        monitor.height_mm
    );
    println!("\n");

    let mut stdout = std::io::stdout();

    // TODO: buffer length = report length (returned by tablet.read)
    let mut buf = [0u8; 10];

    let calculated_x = TABLET_WIDTH / monitor.width_px as f32;
    let calculated_y = TABLET_HEIGHT / monitor.height_px as f32;

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
        let (pen_pos_x, pen_pos_y) = (
            u16::from_be_bytes([buf[3], buf[2]]) as f32,
            u16::from_be_bytes([buf[5], buf[4]]) as f32,
        );

        // Convert pen coordinates to screen
        // (pen_coords - area_offset)/ (tablet_max / screen_max)
        let (screen_pos_x, screen_pos_y) = (
            ((pen_pos_x) / calculated_x) as u16,
            ((pen_pos_y) / calculated_y) as u16,
        );

        // TODO: Elastic tabstops
        println!("{}", data_string);

        fprint!("{}x{}\t", pen_pos_x, pen_pos_y);
        fprint!("{}x{}", screen_pos_x, screen_pos_y);
    }
}
