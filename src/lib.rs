// TODO warnings/etc

pub mod message;

pub use message::bus_id::BusId;
pub use message::message_id::MessageId;
pub use message::payload_length::PayloadLength;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
