use anyhow::Result;
use std::io::{Seek, SeekFrom};

use deku::prelude::*;

#[derive(DekuWrite, PartialEq, DekuRead, Debug)]
pub struct Message {
    pub header: Header,
    #[deku(count = "header.qdcount")]
    pub questions: Vec<Question>,
    #[deku(count = "header.ancount")]
    pub answers: Vec<Answer>,
}

impl Message {
    pub fn new() -> Self {
        Self {
            header: Header::default(),
            questions: vec![],
            answers: vec![],
        }
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            header: Header::default(),
            questions: vec![Question::default(), Question::default()],
            answers: vec![Answer::default(), Answer::default()],
        }
    }
}

#[derive(Clone, DekuWrite, DekuRead, Debug, PartialEq)]
#[deku(endian = "big")]
pub struct Header {
    pub id: u16,
    #[deku(bits = 1)]
    pub qr: u8,
    #[deku(bits = 4)]
    pub opcode: u8,
    #[deku(bits = 1)]
    pub aa: u8,
    #[deku(bits = 1)]
    pub tc: u8,
    #[deku(bits = 1)]
    pub rd: u8,
    #[deku(bits = 1)]
    pub ra: u8,
    #[deku(bits = 3)]
    pub z: u8,
    #[deku(bits = 4)]
    pub rcode: u8,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
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
            qdcount: 2,
            ancount: 2,
            nscount: 0,
            arcount: 0,
        }
    }
}

#[derive(Clone, Debug, DekuWrite, PartialEq, DekuRead)]
#[deku(endian = "big")]
pub struct Question {
    #[deku(
        reader = "read_dns_name(deku::reader)",
        writer = "write_dns_name(deku::writer, &self.name)"
    )]
    pub name: Vec<u8>,
    pub record_type: u16,
    pub class: u16,
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
    result
}

#[derive(DekuWrite, PartialEq, DekuRead, Debug)]
#[deku(endian = "big")]
pub struct Answer {
    #[deku(
        reader = "read_dns_name(deku::reader)",
        writer = "write_dns_name(deku::writer, &self.name)"
    )]
    pub name: Vec<u8>,
    pub record_type: u16,
    pub class: u16,
    pub ttl: u32,
    pub data_length: u16,
    #[deku(count = "data_length")]
    pub data: Vec<u8>,
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

impl Answer {
    pub fn from_question(question: &Question) -> Self {
        Self {
            name: question.name.clone(),
            record_type: question.record_type,
            class: question.class,
            ttl: 60,
            data_length: 4,
            data: vec![8, 8, 8, 8],
        }
    }
}

fn write_dns_name<W: deku::no_std_io::Write + deku::no_std_io::Seek>(
    writer: &mut Writer<W>,
    name: &[u8],
) -> Result<(), DekuError> {
    writer.write_bytes(name)?;
    writer.write_bytes(&[0])?;
    Ok(())
}

fn read_dns_name<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
    reader: &mut Reader<R>,
) -> Result<Vec<u8>, DekuError> {
    let mut name: Vec<u8> = Vec::new();
    loop {
        let mut byte = [0u8; 1];
        reader.read_bytes(1, &mut byte, deku::ctx::Order::Msb0)?;
        let byte = byte[0];
        if byte == 0 {
            break;
        }
        if (byte & 0xc0) == 0xc0 {
            let offset_high = (byte & 0x3f) as u64;
            let mut offset_low = [0u8; 1];
            reader.read_bytes(1, &mut offset_low, deku::ctx::Order::Msb0)?;
            let offset = (offset_high << 8) | offset_low[0] as u64;
            let pos = reader
                .stream_position()
                .map_err(|e| DekuError::Io(e.kind()))?;

            reader
                .seek(SeekFrom::Start(offset))
                .map_err(|e| DekuError::Io(e.kind()))?;

            let suffix = read_dns_name(reader)?;

            reader
                .seek(SeekFrom::Start(pos))
                .map_err(|e| DekuError::Io(e.kind()))?;

            name.extend_from_slice(&suffix);
            break;
        } else {
            name.push(byte);
            let mut label_bytes = vec![0u8; byte as usize];
            reader.read_bytes(byte as usize, &mut label_bytes, deku::ctx::Order::Msb0)?;
            name.extend_from_slice(&label_bytes);
        }
    }
    Ok(name)
}
