#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use codecrafters_dns_server::message::Answer;
    use codecrafters_dns_server::message::Header;
    use codecrafters_dns_server::message::Question;
    use deku::DekuContainerRead;
    use deku::DekuContainerWrite;
    use deku::DekuReader;

    #[test]
    fn test_header_serialize_deserialize() {
        let header = Header::default();
        let bytes = header.to_bytes().unwrap();
        let (rest, deserialized) = Header::from_bytes((&bytes, 0)).unwrap();

        assert_eq!(rest.0.len(), 0);
        assert_eq!(header, deserialized);
    }

    #[test]
    fn test_answer_serialize_deserialize() {
        let answer = Answer::default();
        let serialized = answer.to_bytes().unwrap();
        let mut cursor = Cursor::new(&serialized);
        let mut reader = deku::reader::Reader::new(&mut cursor);
        let deserialized = Answer::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert_eq!(deserialized, answer);
    }

    #[test]
    fn test_question_serialize_deserialize() {
        let question = Question::default();
        let serialized = question.to_bytes().unwrap();
        let mut cursor = Cursor::new(&serialized);
        let mut reader = deku::reader::Reader::new(&mut cursor);
        let deserialized = Question::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert_eq!(deserialized, question);
    }
    #[test]
    fn test_message_serialize_deserialize() {
        use codecrafters_dns_server::message::Message;
        use deku::DekuContainerWrite;

        let message = Message::default();
        let bytes = message.to_bytes().unwrap();
        let mut cursor = Cursor::new(&bytes);
        let mut reader = deku::reader::Reader::new(&mut cursor);
        let deserialized = Message::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert_eq!(message, deserialized);
    }

    #[test]
    fn test_message_roundtrip_with_raw_bytes() {
        use codecrafters_dns_server::message::Message;

        // Example DNS query message for "example.com" type A, class IN
        let raw_bytes: [u8; 29] = [
            0x12, 0x34, // ID
            0x01, 0x00, // Flags: standard query
            0x00, 0x01, // QDCOUNT: 1 question
            0x00, 0x00, // ANCOUNT: 0 answer
            0x00, 0x00, // NSCOUNT: 0 authority
            0x00, 0x00, // ARCOUNT: 0 additional
            0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e', // QNAME: "example"
            0x03, b'c', b'o', b'm', // QNAME: "com"
            0x00, // QNAME: end
            0x00, 0x01, // QTYPE: A
            0x00, 0x01, // QCLASS: IN
        ];

        // Deserialize
        let (rest, msg) =
            Message::from_bytes((&raw_bytes[..], 0)).expect("Failed to deserialize DNS message");
        assert_eq!(rest.0.len(), 0, "Should consume all input bytes");

        // Validate header fields
        assert_eq!(msg.header.id, 0x1234);
        assert_eq!(msg.header.qdcount, 1);
        assert_eq!(msg.header.ancount, 0);
        assert_eq!(msg.header.nscount, 0);
        assert_eq!(msg.header.arcount, 0);

        // Validate question
        assert_eq!(msg.questions.len(), 1);
        let q = &msg.questions[0];
        assert_eq!(
            q.name,
            &[7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm']
        );
        assert_eq!(q.record_type, 1); // A
        assert_eq!(q.class, 1); // IN

        // Serialize
        let serialized = msg.to_bytes().expect("Failed to serialize DNS message");

        // Validate roundtrip: serialized bytes should match original
        assert_eq!(serialized, raw_bytes);
    }
}
