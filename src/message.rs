use deku::prelude::*;

#[derive(Default, DekuRead, DekuWrite, PartialEq)]
pub struct Message {
    header: Header,
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
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }
}
