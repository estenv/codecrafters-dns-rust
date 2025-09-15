use anyhow::Result;
use clap::Parser;
use std::{
    io::Cursor,
    net::{SocketAddrV4, UdpSocket},
};

use codecrafters_dns_server::message::{Answer, Message};
use deku::DekuReader;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    resolver: Option<SocketAddrV4>,
}

fn main() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    let args = Args::parse();

    loop {
        if let Err(e) = read_data(&udp_socket, &mut buf, args.resolver) {
            eprintln!("Error reading data: {}", e);
        }
    }
}

fn read_data(
    udp_socket: &UdpSocket,
    buf: &mut [u8],
    forward_to: Option<SocketAddrV4>,
) -> Result<()> {
    let (_, source) = udp_socket.recv_from(buf)?;
    let response = if let Some(ip_port) = forward_to {
        forward_request(buf, ip_port, udp_socket)?
    } else {
        handle_request(buf)?
    };
    udp_socket.send_to(&response, source)?;
    Ok(())
}

fn forward_request(data: &mut [u8], addr: SocketAddrV4, udp_socket: &UdpSocket) -> Result<Vec<u8>> {
    let request = read_message(data)?;
    let mut responses = vec![];
    let mut requests: Vec<Vec<u8>> = vec![];
    for question in request.questions {
        let mut header = request.header.clone();
        header.qdcount = 1;
        requests.push(
            Message {
                header,
                questions: vec![question.clone()],
                answers: vec![],
            }
            .try_into()?,
        );
    }
    for forwarded_request in requests {
        udp_socket.send_to(&forwarded_request, addr)?;
        let mut buf = [0; 512];
        let (len, _) = udp_socket.recv_from(&mut buf)?;
        responses.push(buf[..len].to_vec());
    }

    let mut res = read_message(&responses[0])?;
    for response in responses.into_iter().skip(1) {
        let ans = read_message(&response)?.answers;
        res.answers.push(ans.into_iter().next().unwrap());
    }
    res.header.ancount = res.answers.len() as u16;
    let res: Vec<u8> = res.try_into()?;
    Ok(res)
}

fn read_message(buf: &[u8]) -> Result<Message, deku::DekuError> {
    let mut cursor = Cursor::new(buf);
    let mut reader = deku::reader::Reader::new(&mut cursor);
    Message::from_reader_with_ctx(&mut reader, ())
}

fn handle_request(data: &[u8]) -> Result<Vec<u8>> {
    let request = read_message(data)?;
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
