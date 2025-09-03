use anyhow::Result;
use std::net::UdpSocket;

use codecrafters_dns_server::message::Message;
use deku::DekuContainerRead;

fn main() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        if let Err(e) = read_data(&udp_socket, &mut buf) {
            eprintln!("Error reading data: {}", e);
        }
    }
}

fn read_data(udp_socket: &UdpSocket, buf: &mut [u8]) -> Result<()> {
    let (_, source) = udp_socket.recv_from(buf)?;
    let response = handle_request(buf)?;
    udp_socket.send_to(&response, source)?;
    Ok(())
}

fn handle_request(data: &[u8]) -> Result<Vec<u8>> {
    let (_rest, request) = Message::from_bytes((data, 0))?;
    let mut message = Message::default();
    message.header.id = request.header.id;
    message.header.opcode = request.header.opcode;
    message.header.rd = request.header.rd;
    message.header.rcode = if request.header.opcode == 0 { 0 } else { 4 };
    message.question.name = request.question.name.clone();
    message.answer.name = request.question.name;

    let response: Vec<u8> = message.try_into().unwrap();
    Ok(response)
}
