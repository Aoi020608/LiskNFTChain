# Data Structures - Stacks and Queues

## Descriptions

1. Stacks

- LIFO last in first out
- browser history
- lookup, pop, push, peek

2. Queues

- FIFO
- lookup, enqueue(add into the queue), dequeue(remove, pop), peek(what is the first item)
- linked list

Line 25, Char 9: cannot borrow `self.elements` as mutable, as it is behind a `&` reference (solution.rs)
|
24 | fn pop(&self) -> i32 {
| ----- help: consider changing this to be a mutable reference: `&mut self`
25 | self.elements.pop_back().unwrap()
| ^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
For more information about this error, try `rustc --explain E0596`.
error: could not compile `prog` due to previous error
mv: cannot stat '/leetcode/rust_compile/target/release/prog': No such file or directory

## Stacks and Queues
1. Pros
- Fast Operations
- Fast Peek
- Ordered

2. Cons
- Slow Lookup
