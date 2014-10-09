extern crate adt;

#[cfg(test)]
mod test {
    use adt::queue::{
        DListQueue,
        Queue
    };

    #[test]
    fn test_new_queue() {
        // Initialize the test queue.
        let queue: DListQueue<int> = DListQueue::new();

        // Verify that the queue is empty.
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_enqueue_queue() {
        // Initialize a test queue.
        let mut queue: DListQueue<int> = DListQueue::new();

        // Queue some elements to the queue.
        queue.enqueue(1);
        queue.enqueue(43);
        queue.enqueue(22);

        // Verify that the queue contains 3 elements.
        assert!(!queue.is_empty());
        assert_eq!(queue.size(), 3);

        // Retrieve a reference to the element at the front of the queue.
        assert_eq!(queue.front().unwrap(), &1);
        assert_eq!(queue.size(), 3);
    }

    #[test]
    fn test_enqueue_then_dequeue_queue() {
        // Initialize a test queue.
        let mut queue: DListQueue<int> = DListQueue::new();

        // Queue some elements to the queue.
        queue.enqueue(1);
        queue.enqueue(43);
        queue.enqueue(22);

        // Verify that the dequeue() function works correctly.
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn test_dequeue_until_empty_queue() {
        // Initialize a test queue.
        let mut queue: DListQueue<int> = DListQueue::new();

        // Queue some elements to the queue.
        queue.enqueue(1);
        queue.enqueue(43);
        queue.enqueue(22);

        // Dequeue the elements off the queue.
        queue.dequeue();
        queue.dequeue();
        queue.dequeue();

        // Verify that the queue is now empty.
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);

        // Retrieve the element from the front of the queue.
        assert_eq!(queue.front(), None);
    }

    #[test]
    fn test_dequeue_empty_queue() {
        // Initialize a test queue.
        let mut queue: DListQueue<int> = DListQueue::new();

        // Dequeue a non-existing element off the queue.
        assert_eq!(queue.dequeue(), None);
    }
}
