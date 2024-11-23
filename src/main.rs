use std::env;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use fake::{
    faker::internet::en::IPv4, faker::internet::en::MACAddress, faker::name::en::Name, Fake,
};
use serde::Serialize;

#[derive(Serialize)]
struct Device {
    id: u32,
    name: String,
    ip: String,
    mac: String,
    location: String,
}

fn generate_mocked_devices(count: usize) -> Vec<Device> {
    let mut devices = Vec::with_capacity(count);

    for i in 0..count {
        let device = Device {
            id: i as u32,
            name: Name().fake(),
            ip: IPv4().fake(),
            mac: MACAddress().fake(),
            location: format!(
                "Building {}, Floor {}",
                fake::rand::random::<u32>() % 10,
                fake::rand::random::<u32>() % 5
            ),
        };
        devices.push(device);
    }

    devices
}

#[get("/")]
async fn hello() -> impl Responder {
    let count: usize = env::var("DEVICE_COUNT")
        .unwrap_or("1000".to_string())
        .parse()
        .unwrap();
    let devices = generate_mocked_devices(count);

    HttpResponse::Ok().json(devices)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap();

    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}