use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    let mut counter = 0;

    loop {
        let variables = [counter, counter + 1, counter + 2];
        let serialized_data = format!("{:?}\n", variables);

        if let Err(e) = stream.write_all(serialized_data.as_bytes()) {
            eprintln!("Failed to send data: {}", e);
            break;
        }

        println!("Sent: {:?}", variables);
        counter += 1;
        thread::sleep(Duration::from_secs(1)); // Delay to avoid spamming
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4000")?;
    println!("Server listening on port 4000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

