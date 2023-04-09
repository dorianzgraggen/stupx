use axum::{
    http::{Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::{
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
    StatusCode::ACCEPTED
}

// the input to our `create_user` handler

#[derive(Deserialize, Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Deserialize, Debug)]
struct PointList {
    points: Vec<Point>,
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
