use std::env;
use std::time::{SystemTime, Duration};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};

// Laika uzglabāšanas objekts
#[derive(Copy, Clone)]
struct TimeStamp {
    ts: u128,
}
impl TimeStamp {
    fn now() -> TimeStamp {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(std::time::UNIX_EPOCH)
            .expect("Unknown error");
        TimeStamp{ ts: since_epoch.as_millis() as u128 }
    }
    fn zero() -> TimeStamp {
        TimeStamp{ ts: 0 }
    }
}

// Servera objekts - satur visas darbības kas ir vajadzīgas servera realizācijai
struct Server {
    date: Arc<Mutex<TimeStamp>>,
    listener: TcpListener,

}
impl Server {
    // Konstruktors
    fn new(address: String) -> Server {
        let listener = TcpListener::bind(address).unwrap();

        Server {
            date: Arc::new(Mutex::new(TimeStamp::zero())),
            listener: listener,
        }
    }

    // Šo funkciju palaiž katrs savienojums. Izvada pašreizējo laiku milisekundēs.
    fn handle_client(mut stream: TcpStream, date: Arc<Mutex<TimeStamp>>) {
        let mut buffer = [0 as u8; 1];
        stream.read(&mut buffer).unwrap();
        let current_date = *date.lock().unwrap();
        stream.write(&current_date.ts.to_be_bytes()).unwrap();
    }

    // Atjauninam laiku
    fn update_date(date: Arc<Mutex<TimeStamp>>) {
        let new_date = TimeStamp::now();
        if let Ok(mut date) = date.lock() {
            *date = new_date;
        }
    }

    // Tiek lietots lai sāktu serveri.
    fn run(&self) {
        // Serveri gaida cits thread, main thread notiek datu atjaunināšana
        // Saglabājam klonētās vērtības mainīgajos, jo Rust nepatīk iespēja, ka thread
        // varētu dzīvot ilgāk par self objektu.
        let cloned_date = self.date.clone();
        let cloned_listener = self.listener.try_clone().unwrap();
        let connection_handler = thread::spawn(move || { &Server::process_requests(cloned_listener, cloned_date); });

        // Regulāri atjaunina laiku
        let quit = false;
        while ! quit {
            Server::update_date(self.date.clone());
            thread::sleep(Duration::new(1,0));
        }
        connection_handler.join().unwrap();
    }

    // Funkcija kas tiek lietota lai apstrādātu individuālus savienojumus
    fn process_requests(listener: TcpListener, date: Arc<Mutex<TimeStamp>>) {
        for stream in listener.incoming() { // Ekvivalents while() loopam, iz dokumentācijas
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let cloned_date = date.clone();
                    thread::spawn(|| { Server::handle_client(stream, cloned_date); });
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
