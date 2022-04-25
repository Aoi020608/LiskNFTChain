/*

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

// type Link<T> = Option<Box<Node<T>>>;

// struct Node<T> {
//     elem: T,
//     next: Link<T>,
// }

// pub struct List<T> {
//     head: Link<T>,
//     tail: Link<T>,
//     length: u32,
// }

// impl<T> List<T> {
//     pub fn new() -> Self {
//         List {
//             head: None,
//             tail: None,
//             length: 0,
//         }
//     }

//     pub fn append(&mut self, elem: T) {
//         let new_node = Box::new(Node { elem, next: None });
//         match self.tail {
//             None => {
//                 self.head = Some(new_node);
//                 self.tail = Some(new_node);
//             },
//             Some(ref mut node) => {
//                 node.next = Some(new_node);
//                 node = Some();
//             }
//         }
//         self.length += 1;
//         self.tail = Some(new_node);
//     }

//     pub fn push(&mut self, elem: T) {
//         let new_node = Box::new(Node {
//             elem,
//             next: self.head.take(),
//         });
//         self.head = Some(new_node);
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         self.head.take().map(|node| {
//             self.head = node.next;
//             node.elem
//         })
//     }

//     pub fn peek(&self) -> Option<&T> {
//         self.head.as_ref().map(|node| &node.elem)
//     }
// }

#[derive(Debug, PartialEq)]
struct ListNodeValue<T> {
    item: T,
    next: Box<ListNode<T>>,
}

impl<T> ListNodeValue<T> {
    fn new(item: T, next: Box<ListNode<T>>) -> Self {
        Self { item, next }
    }
}

// where allows specifying constraints on lifetime and generic parameters.
impl<T> Clone for ListNodeValue<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            item: self.item.clone(),
            next: Box::clone(&self.next),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ListNode<T> {
    Empty,
    NonEmpty(ListNodeValue<T>),
}

impl<T> Default for ListNode<T> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<T> ListNode<T> {
    fn new(item: T, next: Box<ListNode<T>>) -> Self {
        Self::NonEmpty(ListNodeValue::new(item, next))
    }

    fn take(&mut self) -> Self {
        let mut cur = Self::Empty;
        std::mem::swap(&mut cur, self);
        cur
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SinglyLinkedList<T> {
    head: Box<ListNode<T>>,
    size: usize,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: Box::new(ListNode::Empty),
            size: 0,
        }
    }

    pub fn push_front(&mut self, item: T) {
        let cur_head = self.head.take();
        let new_node = Box::new(ListNode::new(item, Box::new(cur_head)));

        self.head = new_node;
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // take ownershiop of head
        let head = self.head.take();
        match head {
            ListNode::NonEmpty(node) => {
                self.head = node.next;
                self.size -= 1;
                Some(node.item)
            }
            ListNode::Empty => None,
        }
        // self.size -= 1;
        // if let ListNode::NonEmpty(node) = head {
        //     self.head = node.next;
        //     self.size -= 1;
        //     Some(node.item)
        //     // println!("{:?}", node.item);
        //     // None
        // } else {
        //     None
        // }
    }

    pub fn insert_after(&mut self, pos: usize, elem: T) -> Result<(), usize> {
        let mut curr = &mut self.head;
        let mut pos_ = pos;

        while pos_ > 0 {
            curr = match curr.as_mut() {
                ListNode::NonEmpty(node) => &mut node.next,
                ListNode::Empty => return Err(pos - pos_),
            };
            pos_ -= 1;
        }

        match curr.take() {
            ListNode::NonEmpty(mut node) => {
                let new_node = Box::new(ListNode::new(elem, node.next));
                node.next = new_node;
                *curr = Box::new(ListNode::NonEmpty(node));
            }
            ListNode::Empty => return Err(pos - pos_),
        }

        Ok(())
    }

    pub fn remove(&mut self, pos_: usize) -> Result<(), usize> {
        let mut curr = &mut self.head;
        let mut pos = pos_;

        while pos > 0 {
            // let curr = &mut curr.as_mut();
            curr = match curr.as_mut() {
                ListNode::NonEmpty(node) => &mut node.next,
                ListNode::Empty => return Err(pos),
            };

            pos -= 1;
        }

        match curr.take() {
            ListNode::NonEmpty(node) => {
                *curr = node.next;
                self.size -= 1;
            }
            ListNode::Empty => return Err(pos),
        }

        Ok(())
    }

    pub fn reverse(&mut self) -> Result<(), String>{
        let mut curr = &mut self.head;
        let size = self.size;

        while size >= 1 {
            curr = match curr.as_mut() {
                ListNode::NonEmpty(node) => {
                    // node.next = curr;
                    let node_next = &mut node.next;

                    node_next
                }
                ListNode::Empty => {
                    return Err("Error".to_string());
                }
            }
        }

        Ok(())
    }

    pub fn append(&mut self, item: T) {
        for i in 0..self.size {}
    }
}

*/

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
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

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_ref().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
