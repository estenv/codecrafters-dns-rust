use deku::prelude::*;

#[derive(Default, DekuWrite, PartialEq)]
pub struct Message {
    header: Header,
    question: Question,
    answer: Answer,
}

#[derive(DekuWrite, DekuRead, Debug, PartialEq)]
#[deku(endian = "big")]
pub struct Header {
    id: u16,
    #[deku(bits = 1)]
    qr: u8,
    #[deku(bits = 4)]
    opcode: u8,
    #[deku(bits = 1)]
    aa: u8,
    #[deku(bits = 1)]
    tc: u8,
    #[deku(bits = 1)]
    rd: u8,
    #[deku(bits = 1)]
    ra: u8,
    #[deku(bits = 3)]
    z: u8,
    #[deku(bits = 4)]
    rcode: u8,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            id: 1234,
            qr: 1,
            opcode: 0,
            aa: 0,
            tc: 0,
            rd: 0,
            ra: 0,
            z: 0,
            rcode: 0,
            qdcount: 1,
            ancount: 1,
            nscount: 0,
            arcount: 0,
        }
    }
}

#[derive(DekuWrite, PartialEq)]
#[deku(endian = "big")]
pub struct Question {
    name: Vec<u8>,
    record_type: u16,
    class: u16,
}

impl Default for Question {
    fn default() -> Self {
        Self {
            name: encode_label(&["codecrafters", "io"]),
            record_type: 1,
            class: 1,
        }
    }
}

fn encode_label(labels: &[&str]) -> Vec<u8> {
    let mut result = vec![];
    for label in labels {
        result.push(label.len() as u8);
        result.extend_from_slice(label.as_bytes());
    }
    result.push(0);
    result
}

#[derive(DekuWrite, PartialEq)]
#[deku(endian = "big")]
pub struct Answer {
    name: Vec<u8>,
    record_type: u16,
    class: u16,
    ttl: u32,
    data_length: u16,
    data: Vec<u8>,
}

impl Default for Answer {
    fn default() -> Self {
        Self {
            name: encode_label(&["codecrafters", "io"]),
            record_type: 1,
            class: 1,
            ttl: 60,
            data_length: 4,
            data: vec![8, 8, 8, 8],
        }
    }
}
