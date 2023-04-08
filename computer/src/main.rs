use std::sync::mpsc::channel;
use std::thread;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};
fn main() {
    // process_test();

    let (tx, rx) = channel();

    let sender = thread::spawn(move || {
        let mut child = Command::new("ping")
            .args(["example.com"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        let stdout = child.stdout.take().unwrap();

        // Stream output.
        let lines = BufReader::new(stdout).lines();
        for line in lines {
            // println!("lol: {}", line.unwrap());
            tx.send(line.unwrap()).expect("Unable to send on channel");
        }
    });

    let receiver = thread::spawn(move || loop {
        // sleep(Duration::from_millis(100));
        // let value = rx.recv();
        if let Ok(value) = rx.recv() {
            println!("aha {}", value);
        } else {
            println!("no message");
            break;
        }
    });

    sender.join().expect("The sender thread has panicked");
    receiver.join().expect("The receiver thread has panicked");
}

fn process_test() {
    let mut child = Command::new("ping")
        .args(["example.com"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let stdout = child.stdout.take().unwrap();

    // Stream output.
    let lines = BufReader::new(stdout).lines();
    for line in lines {
        println!("lol: {}", line.unwrap());
    }
}

fn serial_test() -> ! {
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
