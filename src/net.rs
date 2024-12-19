use std::thread::sleep;
use std::thread;
use std::net::UdpSocket;
use std::time::Duration;
use std::sync::mpsc::{channel, Sender};
use std::collections::VecDeque;


pub fn network() -> Sender<Vec<i64>> {
    let (tx, rx) = channel();
    let tx: Sender<Vec<i64>> = tx.clone();

    thread::spawn( move || { 
        let mut net_active = 1;
        while net_active == 1 {
            let server_socket = UdpSocket::bind("0.0.0.0:0").unwrap();
            let target = "100.127.105.90:4000";
            let dat = rx.recv().unwrap();



            if dat[0] < 0 {
                eprintln!("Invalid code (negative code): {}" ,dat[0])

            }

            if dat[0] == 0 { // code 0, kills thread
                net_active = 0;

            }
            if dat[0] == 1 {
                sleep(Duration::from_secs(1800)); // code 1, thread sleeps for 30 minutes 

            }

            if dat[0] == 2 { // code 2, sends data to server
                if dat[1] > 0 && dat[2] > 0 && dat[3] > 0 {
                    let mut aa = VecDeque::from(dat);
                    aa.pop_front();
                    let serialized_data = format!("{:?}\n", aa);

                    println!("Sending scores and upgrade data\n
                              data is: {}"
                              ,serialized_data);

                    if let Err(e) = server_socket.send_to(serialized_data.as_bytes(), target) {
                    eprintln!("Failed to send data: {}" ,e)
                    }
                }
            }
        }
    });

    tx
}

pub fn network_disable(tx: Sender<Vec<i64>>) -> Sender<Vec<i64>> {
    let kill = [0, 0].to_vec();
    let _ = tx.send(kill);
    tx
}

pub fn send_score(tx: Sender<Vec<i64>>, x: i32, upgrd_cost: i32, score: i32) -> Sender<Vec<i64>> {
    let send: [i64; 4] = [2, x.into(), upgrd_cost.into(), score.into()];
    let _ = tx.send(send.to_vec());
    tx
}

pub fn debug(tx: Sender<Vec<i64>>) -> Sender<Vec<i64>> {
    let send: [i64; 5] = [2, 65565, 65565, 65565, 1337];
    let _ = tx.send(send.to_vec());
    tx
}


