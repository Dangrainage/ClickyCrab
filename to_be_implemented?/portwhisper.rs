use std::net::UdpSocket;
use std::thread::sleep;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Create a UDP socket bound to port 4000
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let target = "127.0.0.1:4000"; // Target address and port

    // Data to send
    let data = [1, 2, 3, 4, 5]; // Example array of variables

    println!("Sending data to {}...", target);

    loop {
        // Serialize the data into a byte string
        let serialized_data = format!("{:?}\n", data);
        // Send the data to the target
        if let Err(e) = socket.send_to(serialized_data.as_bytes(), target) {
            eprintln!("Failed to send data: {}", e);
        }

        // Pause for 1 second
        sleep(Duration::from_secs(1));
    }
}
