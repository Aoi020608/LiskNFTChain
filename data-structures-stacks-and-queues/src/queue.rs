// use dyn_clone::DynClone;
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

// impl Clone for Box<T> {
//     fn clone(&self) -> Box<T> {
//         self.clone()
//     }
// }

#[derive(Debug)]
struct Queue<T> {
    top: Link<T>,
    bottom: Link<T>,
    length: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            top: None,
            bottom: None,
            length: 0,
        }
    }

    pub fn peek(&self) -> Option<T> {
        None
    }

    pub fn enqueue(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.top.take(),
        });
        match self.top.take() {
            None => {
                self.bottom = Some(new_node);
            }
            Some(node) => {
                new_node.next = self.top;
            }
        }
        self.top = Some(new_node);
        self.length += 1;
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        Err("Not implemeted")
    }
}

#[cfg(test)]
mod queue_test {
    use crate::queue::*;
    #[test]
    fn basis() {
        let mut list = Queue::new();
        list.enqueue(1);
        println!("{:?}", list);
    }
}
