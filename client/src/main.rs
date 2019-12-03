use std::env;
use std::net::{TcpStream};
use std::io::{Read, Write};

fn main() {
    // Argumentu apstrāde
    let args: Vec<String> = env::args().collect();

    // Pārbaudam vai ir visi argumenti
    assert_eq!(args.len(), 3, "Required arguments: ip port");

    let addr = &args[1];
    let port = &args[2];
    let address: String = format!("{}:{}", addr, port);

    // Veic savienojumu un izvada atbildi
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            stream.write(b"1").unwrap();

            let mut buffer = [0 as u8; 16];
            match stream.read_exact(&mut buffer) {
                Ok(_) => {
                    println!("{}", u128::from_be_bytes(buffer).to_string());
                },
                Err(e) => {
                    println!("Could not receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Could not connect: {}", e);
        }
    }
}
