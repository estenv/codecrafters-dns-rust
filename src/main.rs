#[allow(unused_imports)]
use std::net::UdpSocket;

use codecrafters_dns_server::message::{Header, Message};
use deku::DekuContainerRead;

fn main() {
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let Ok((_, request)) = Header::from_bytes((&buf, 0)) else {
                    continue;
                };
                let mut message = Message::default();
                message.header.id = request.id;
                message.header.opcode = request.opcode;
                message.header.rd = request.rd;
                message.header.rcode = if request.opcode == 0 { 0 } else { 4 };
                dbg!(&message.header.id);
                let response: Vec<u8> = message.try_into().unwrap();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
