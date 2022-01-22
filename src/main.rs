use hidapi::HidApi;
use std::io::Write;

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

    // TODO: buffer length = report length (returned by tablet.read)
    let mut buf = [0u8; 10];

    // TODO: Read at devide frequency
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

        // Clear last line and print
        print!("{}[2K", 27 as char);
        print!("\r{}", data_string);
        std::io::stdout().flush().expect("Failed to flush");
    }
}
