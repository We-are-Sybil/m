pub trait Message {
    /// Get the recipient's phone number in E.164 format
    fn recipient(&self) -> &str;

    /// Get the message type identifier
    fn message_type(&self) -> &str;
}
