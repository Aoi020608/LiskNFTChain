// 10 --> 5 --> 16
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::singly_linked_list::*;
use crate::doubly_linked_list::*;

mod singly_linked_list;
mod doubly_linked_list;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    length: u32,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    // Act like we own boxed nodes since we construct and leak them
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            marker: PhantomData,
        }
    }

    pub fn insert_at_head(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = self.head;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.head {
            None => self.tail = node_ptr,
            _ => {}
        }
        self.head = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.head, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}

fn main() {
    // let mut my_linked_list = LinkedList::<String>::new();
    // my_linked_list.insert_at_head("Hello".to_string());
    // my_linked_list.insert_at_head("world!".to_string());

    // match my_linked_list.get(0) {
    //     None => panic!("Expected to find {} at index 1", "Hello"),
    //     Some(val) => println!("{:?}", val),
    // }

    // let mut linked_list: SinglyLinkedList<usize> = SinglyLinkedList::new();
    // for i in 0..5 {
    //     linked_list.push_front(i);
    // }

    // // linked_list.pop_front();
    // let result_insert = linked_list.insert_after(1, 99);
    // let result_remove = linked_list.remove(1);
    // println!("{:?}", result_insert);
    // println!("{:?}", result_remove);

    // println!("{:?}", linked_list);

    // let mut doubly_list: DoublyLinkedList<usize> = DoublyLinkedList::new();

    // for i in 0..5 {
    //     doubly_list.push_front(i)
    // }

    // println!("{:?}", doubly_list.len());
    
    let mut list = List::new();
    list.push(1);
    list.push(34);
    list.push(89);

    list.peek().map(|node| println!("{}", node));

    let mut iter = list.into_iter();
    let number = iter.next().unwrap();
    println!("{}", number);

}
