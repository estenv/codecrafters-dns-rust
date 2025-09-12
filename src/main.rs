use anyhow::Result;
use std::{io::Cursor, net::UdpSocket};

use codecrafters_dns_server::message::{Answer, Message};
use deku::DekuReader;

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
    let mut cursor = Cursor::new(data);
    let mut reader = deku::reader::Reader::new(&mut cursor);
    let request = Message::from_reader_with_ctx(&mut reader, ())?;
    let mut response = Message::new();
    for question in &request.questions {
        response.questions.push(question.clone());
        response.answers.push(Answer::from_question(question));
    }
    response.header.id = request.header.id;
    response.header.opcode = request.header.opcode;
    response.header.rd = request.header.rd;
    response.header.rcode = if request.header.opcode == 0 { 0 } else { 4 };
    response.header.qdcount = response.questions.len() as u16;
    response.header.ancount = response.answers.len() as u16;

    let response: Vec<u8> = response.try_into().unwrap();
    Ok(response)
}
