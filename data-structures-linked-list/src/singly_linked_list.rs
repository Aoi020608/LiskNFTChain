// use std::clone::Clone;
// // Box is called a "smart pointer"
// #[derive(Debug, Clone)]
// struct Node<T> {
//     data: T,
//     next: Option<Box<Node<T>>>,
// }

// impl<T> Node<T> {
//     pub fn new(obj: T) -> Self {
//         Node {
//             data: obj,
//             next: None,
//         }
//     }

//     pub fn append(&mut self, new_node: Box<Node<T>>) {
//         self.next = Some(new_node.clone());
//     }
// }

// #[derive(Debug)]
// struct LinkedList<T> {
//     length: u32,
//     head: Option<Box<Node<T>>>,
//     tail: Option<Box<Node<T>>>,
// }

// impl<T> LinkedList<T> {
//     pub fn new() -> LinkedList<T> {
//         LinkedList {
//             length: 0,
//             head: None,
//             tail: None,
//         }
//     }

//     pub fn push(&mut self, element: T) {
//         let new_node: Box<Node<T>> = Box::new(Node::new(element));

//         // match self.tail {
//         //     None => self.head = Some(new_node),
//         //     Some(ref mut tail) => tail.append(new_node),
//         // }
//         // self.tail = Some(new_node);
//         // self.length += 1;
//     }
// }

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    length: u32,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, elem: T) {
        let new_node = Box::new(Node { elem, next: None });
        
        self.tail = Some(new_node);
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}
