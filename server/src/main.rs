use std::env;
use std::time::{SystemTime};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


// Servera objekts - satur visas darbības kas ir vajadzīgas servera realizācijai
struct Server {
    listener: TcpListener,

}
impl Server {
    // Konstruktors
    fn new(address: String) -> Server {
        let listener = TcpListener::bind(address).unwrap();

        Server {
            listener: listener,
        }
    }

    // Šo funkciju palaiž katrs savienojums. Izvada pašreizējo laiku milisekundēs.
    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0 as u8; 1];
        stream.read(&mut buffer).unwrap();

        let now = SystemTime::now();
        let since_epoch = now.duration_since(std::time::UNIX_EPOCH)
                .expect("Unknown error");
        stream.write(&since_epoch.as_millis().to_be_bytes()).unwrap();
    }

    // Tiek lietots lai sāktu serveri.
    fn run(&self) {
        self.process_requests();
    }

    // Funkcija kas tiek lietota lai apstrādātu individuālus savienojumus
    fn process_requests(&self) {
        for stream in self.listener.incoming() { // Ekvivalents while() loopam, iz dokumentācijas
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    self.handle_client(stream);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}

fn main() {
    // Argumentu apstrāde
    let args: Vec<String> = env::args().collect();

    // Pārbaudam vai ir visi argumenti
    assert_eq!(args.len(), 3, "Required arguments: ip port");

    let addr = &args[1];
    let port = &args[2];

    let address: String = format!("{}:{}", addr, port);

    let server = Server::new(address);
    server.run();
}
