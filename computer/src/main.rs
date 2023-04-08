use std::time::Duration;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("COM5", 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    // let output = "This is a test. This is only a test.".as_bytes();
    // port.write(output).expect("Write failed!");

    loop {
        let mut serial_buf: Vec<u8> = vec![0; 32];
        port.read_exact(serial_buf.as_mut_slice())
            .expect("Found no data!");
    }
}
