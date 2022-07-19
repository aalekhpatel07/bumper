mod structs;

pub use structs::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut buffer: RingBuffer<usize> = RingBuffer::new(10);
        let (mut producer, mut consumer) = buffer.split();
        producer.push(1).unwrap();
        producer.push(2).unwrap();
        producer.push(3).unwrap();

        assert_eq!(consumer.pop().unwrap(), 1);
        assert_eq!(consumer.pop().unwrap(), 2);
        assert_eq!(consumer.pop().unwrap(), 3);
    }
}
