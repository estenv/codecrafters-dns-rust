#[cfg(test)]
mod tests {
    use codecrafters_dns_server::message::Answer;
    use codecrafters_dns_server::message::Header;
    use codecrafters_dns_server::message::Question;
    use deku::DekuContainerRead;
    use deku::DekuContainerWrite;

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
        let (rest, deserialized) = Answer::from_bytes((&serialized, 0)).unwrap();

        assert_eq!(rest.0.len(), 0);
        assert_eq!(deserialized, answer);
    }

    #[test]
    fn test_question_serialize_deserialize() {
        let question = Question::default();
        let serialized = question.to_bytes().unwrap();
        let (rest, deserialized) = Question::from_bytes((&serialized, 0)).unwrap();

        assert_eq!(rest.0.len(), 0);
        assert_eq!(deserialized, question);
    }
}
