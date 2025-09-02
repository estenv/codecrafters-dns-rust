#[cfg(test)]
mod tests {
    use codecrafters_dns_server::message::Header;
    use deku::DekuContainerRead;
    use deku::DekuContainerWrite;

    #[test]
    fn test_header_serialize_deserialize() {
        let header = Header {
            id: 0xABCD,
            qr: 1,
            opcode: 0,
            aa: 1,
            tc: 0,
            rd: 1,
            ra: 1,
            z: 0,
            rcode: 0,
            qdcount: 1,
            ancount: 2,
            nscount: 0,
            arcount: 0,
        };

        // Serialize
        let bytes = header.to_bytes().unwrap();

        // Deserialize
        let (rest, deserialized) = Header::from_bytes((&bytes, 0)).unwrap();

        assert_eq!(rest.0.len(), 0);
        assert_eq!(header, deserialized);
    }
}
