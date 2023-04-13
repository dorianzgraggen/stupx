use axum::{
    http::{Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::{
    default,
    f32::consts::{PI, TAU},
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    time::{Duration, SystemTime},
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
        .route("/reset", post(reset))
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

async fn reset() -> StatusCode {
    std::thread::spawn(move || {
        send_painting_commands(PointList {
            points: vec![Point::new(0.0, 0.0)],
        })
    });
    StatusCode::ACCEPTED
}

#[derive(Deserialize, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn angle_rad(&self) -> f32 {
        self.x.atan2(self.y) + PI
    }

    fn angle_remapped_256(&self) -> u8 {
        ((256.0 / TAU) * self.angle_rad()) as u8
    }

    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
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
        let remapped_angle = point.angle_remapped_256();

        // std::thread::sleep(Duration::from_millis(40));

        println!(
            "Sending length={}, angle={} ({}/{})",
            remapped_length,
            remapped_angle,
            i,
            list.points.len()
        );
        let _amount = port
            .write(&[remapped_length, remapped_angle])
            .expect("Write failed!");
        // let _amount = port.write(&[remapped_angle]).expect("Write failed!");

        let mut waiting = true;
        let time_start = SystemTime::now();

        while waiting {
            let num = port.bytes_to_read().unwrap();
            if num > 0 {
                port.read_exact(serial_buf.as_mut_slice())
                    .expect("Found no data!");

                let byte = &serial_buf[0];
                match &serial_buf[0] {
                    65u8 => {
                        println!("A: Serial available");
                    }
                    66u8 => {
                        println!("B: Ready for move");
                    }
                    67u8 => {
                        println!("C: finished moving");
                        waiting = false;
                    }
                    68u8 => {
                        println!("D: looping");
                    }
                    69u8 => {
                        println!("E: waiting_for_top_pos");
                    }
                    70u8 => {
                        println!("F: waiting_for_rot");
                    }
                    _ => {
                        println!("-- no matching command for {}", byte);
                    }
                }
                // println!("{:#?}", &serial_buf[0]);
            }

            if time_start.elapsed().unwrap().as_millis() > 200 {
                println!("skipping");
                waiting = false;
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
