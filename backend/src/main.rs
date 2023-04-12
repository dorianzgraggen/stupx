use axum::{
    http::{Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::{
    default,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    time::Duration,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/draw", post(draw))
        .layer(cors);

    println!("letsago");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn draw(Json(payload): Json<PointList>) -> StatusCode {
    println!("{:#?}", payload);
    std::thread::spawn(move || send_painting_commands(payload));
    StatusCode::ACCEPTED
}

// the input to our `create_user` handler

#[derive(Deserialize, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[derive(Deserialize, Debug)]
struct PointList {
    points: Vec<Point>,
}

fn send_painting_commands(list: PointList) {
    let mut max = 0.0;

    let measured_max_length = 18.0;

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("COM5", 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    std::thread::sleep(Duration::from_millis(4000));

    let mut serial_buf: Vec<u8> = vec![0; 1];

    for (i, point) in list.points.iter().enumerate() {
        // sends signal that length is arriving
        let output = "h".as_bytes();
        let _amount = port.write(output).expect("Write failed!");

        let length = point.length();
        let remapped_length = (length * (256.0 / 18.0)) as u8;
        println!(
            "Sending {}, originally {} ({}/{})",
            remapped_length,
            length,
            i,
            list.points.len()
        );
        let _amount = port.write(&[remapped_length]).expect("Write failed!");

        let mut waiting = true;
        while waiting {
            let num = port.bytes_to_read().unwrap();
            if num > 0 {
                port.read_exact(serial_buf.as_mut_slice())
                    .expect("Found no data!");

                let byte = &serial_buf[0];
                match &serial_buf[0] {
                    65u8 => {
                        // A
                        println!("Still moving");
                    }
                    66u8 => {
                        // B
                        println!("Ready for move");
                    }
                    67u8 => {
                        // C
                        println!("C: finished moving");
                        waiting = false;
                    }
                    _ => {
                        println!("-- no matching command for {}", byte);
                    }
                }
                // println!("{:#?}", &serial_buf[0]);
            }
        }
    }
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
