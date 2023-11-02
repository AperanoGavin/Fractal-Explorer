use std::env;
extern crate image;

mod complex;
mod fractal;

use fractal::{mandelbrot, color};

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 8]; // On suppose que les données envoyées seront de type u32 (4 octets pour chaque valeur de largeur et hauteur)
    stream.read_exact(&mut buffer).expect("Erreur de lecture des données");

    let image_width = u32::from_ne_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
    let image_height = u32::from_ne_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);

    println!("Largeur de l'image : {}", image_width);
    println!("Hauteur de l'image : {}", image_height);
    // Faites ce que vous voulez avec les données reçues (exécutez la génération d'image, etc.)

    let mut image_buffer = image::ImageBuffer::new(
        image_width, image_height);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let u = x as f32 / image_height as f32;
        let v = y as f32 / image_height as f32;
        let t = mandelbrot(2.5 * (u - 0.5) - 1.4, 2.5 * (v - 0.5));
        *pixel = image::Rgb(color((2.0 * t + 0.5) % 1.0));
    }

    image_buffer.save("mandelbrot.png").unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Impossible de lier l'adresse");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Erreur de connexion : {}", e);
            }
        }
    }
}
