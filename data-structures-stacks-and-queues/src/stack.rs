use std::cell::RefCell;
use std::rc::Rc;
// RC - multiple owner
// RefCell - modified

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

struct Stack<T> {
    top: Link<T>,
    bottom: Link<T>,
    length: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: None,
            bottom: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        if let Some(prev_top) = self.top.take() {
            prev_top.borrow_mut().next = Some(Rc::clone(&new_node));
            self.top = Some(Rc::clone(&new_node));
            self.length += 1;
        } else {
            self.top = Some(Rc::clone(&new_node));
            self.bottom = Some(Rc::clone(&new_node));
            self.length += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|prev_top| {
            self.length -= 1;
            match prev_top.borrow_mut().next.take() {
                Some(node) => {
                    self.top = Some(node);
                }
                None => {
                    self.bottom.take();
                }
            }
            Rc::try_unwrap(prev_top).ok().unwrap().into_inner().data
        })
    }

    pub fn peek(&mut self) -> Option<RefCell<Node<T>>> {
        // match self.top.as_ref() {
        //     None => None,
        //     Some(node) => {
        //         let clone_node = Rc::clone(&node);
        //         Rc::try_unwrap(clone_node).ok().unwrap().into_inner().data
        //     }
        // }
        // self.top.take() RefCell
        // if let Some(top_node) = self.top.take() {
        //     Rc::try_unwrap(top_node).ok()
        // } else {
        //     None
        // }
        // self.top.take() Data; RefCell { value: Node { data: 2, next: None } }
        match self.top.take() {
            None => None,
            Some(ref_cell) => {
                Some(ref_cell.borrow_mut().)
            }
        }
        // let clone_node = Rc::clone(self.top.take());
        // Rc::try_unwrap(clone_node).ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::*;
    #[test]
    fn push_test() {
        let mut stack = Stack::new();
        stack.push(1);
        let length = stack.len();
        assert_eq!(length, 1);
    }

    #[test]
    fn pop_test() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        let length = stack.len();
        assert_eq!(length, 2);
        let data = stack.pop().unwrap();
        let length = stack.len();
        assert_eq!(length, 1);
        assert_eq!(data, 2);
    }

    #[test]
    fn peek_test() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        // let length = stack.len();
        // assert_eq!(length, 2);
        let data = stack.peek().expect("Found error");
        println!("Data; {:?}", data);
        // assert_eq!(data, 2);
    }
}
