use std::collections::LinkedList;

#[derive(Debug)]
pub struct Queue<T> {
    elements: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            elements: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.elements.front()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}

#[cfg(test)]

mod tests {
    use crate::queue_using_crate::*;
    #[test]
    fn test_enqueue() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(40);
        assert_eq!(queue.peek_front(), Some(&40));
    }

    #[test]
    fn test_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        let retrieved_dequeue = queue.dequeue();
        assert_eq!(retrieved_dequeue, Some(1));
    }

    #[test]
    fn test_peek_front() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        let peeked_queue = queue.peek_front();
        assert_eq!(peeked_queue, Some(&1));
    }

    #[test]
    fn test_size() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.len(), 2);
    }
}
