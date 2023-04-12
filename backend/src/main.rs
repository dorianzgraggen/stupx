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

    let output = "h".as_bytes();
    println!("{:#?}", output);
    let _amount = port.write(output).expect("Write failed!");

    // for point in list.points {
    //     let length = point.length();
    //     let remapped_length = (length * (256.0 / 18.0)) as u8;
    //     println!("Sending {}, originally {}", remapped_length, length);
    //     let _amount = port.write(&[remapped_length]).expect("Write failed!");

    //     std::thread::sleep(Duration::from_millis(2000));
    // }

    // return;

    let mut waiting = false;
    let mut serial_buf: Vec<u8> = vec![0; 1];

    loop {
        let num = port.bytes_to_read().unwrap();
        if num > 0 {
            waiting = false;
            port.read_exact(serial_buf.as_mut_slice())
                .expect("Found no data!");
            // println!("{:#?}", &serial_buf[..]);

            let v = &serial_buf[0];
            match v {
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
                    println!("C");
                }
                _ => {
                    println!("-- no matching command for {}", v);
                }
            }
            // println!("{:#?}", &serial_buf[0]);
        } else if !waiting {
            waiting = true;
            println!("-- now waiting");
        }
        // println!(
        //     "{:#?}",
        //     &serial_buf[..].iter().for_each(|item, iter| { char::from })
        // );

        // match std::str::from_utf8(&serial_buf[..bytes]) {
        //     Ok(s) => println!("[arduino] {}", s),
        //     Err(e) => {
        //         println!("Error: {}", e);
        //         println!("Data: {:#?}", serial_buf);
        //         panic!("aaaaah");
        //     }
        // }
        // std::thread::sleep(Duration::from_millis(100))
    }

    // let mut reader = BufReader::new(port);
    // let mut my_str = String::new();

    // loop {

    //     reader.read_line(&mut my_str);

    //     println!("{}", my_str);
    // }
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
