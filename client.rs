extern crate dialoguer;

use dialoguer::Input;
use std::net::TcpStream;
use std::io::Write;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Impossible de se connecter au serveur");

    //let image_width: u32 = 1920; 
    //let image_height: u32 = 1080; 
    let image_width: u32 = Input::new()
    .with_prompt("Entrez la largeur de l'image")
    .interact()
    .unwrap();

    let image_height: u32 = Input::new()
        .with_prompt("Entrez la hauteur de l'image")
        .interact()
        .unwrap();

    let data = [image_width.to_ne_bytes(), image_height.to_ne_bytes()].concat();
    stream.write_all(&data).expect("Erreur d'envoi des données");
}
