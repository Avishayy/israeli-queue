mod israeliqueue;
pub use israeliqueue::IsraeliQueue;

#[cfg(test)]
mod tests {
    pub use IsraeliQueue;

    #[test]
    fn init() {
        let iq: IsraeliQueue<i32> = IsraeliQueue::new();
        assert!(iq.is_empty());
    }

    fn test_comparator(a: &i32, b: &i32) -> bool {
        a == b
    }

    #[test]
    fn queue_dequeue() {
        let mut iq: IsraeliQueue<i32> = IsraeliQueue::new();
        iq.queue(15, Some(&test_comparator));
        iq.queue(1, Some(&test_comparator));
        iq.queue(13, Some(&test_comparator));
        iq.queue(15, Some(&test_comparator));
        assert!(!iq.is_empty());
        assert_eq!(15, *iq.peek().unwrap());
        assert_eq!(15, iq.dequeue().unwrap());
        assert_eq!(15, iq.dequeue().unwrap());
        assert_eq!(1, iq.dequeue().unwrap());
        assert_eq!(13, iq.dequeue().unwrap());
    }
}
