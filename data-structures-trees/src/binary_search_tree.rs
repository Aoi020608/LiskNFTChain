/// Level 0: 2^0 = 1 total of nodes in one line
/// Level 1: 2^1 = 2
/// Level 2: 2^2 = 4
/// Level 3: 2^3 = 6
/// Level 4: 2^4 = 8
/// 
/// 

struct Node{
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: i32,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            left: None,
            right: None,
            value
        }
    }

    pub fn insert(&mut self, value: i32) {
        if self.value > value {
            match self.left {
                None => self.left = Some(Box::new(Node{value, left: None, right: None})),
                Some(ref mut node) => node.insert(value)
            }
        } else {
            match self.right {
                None => self.right = Some(Box::new(Node {value, left: None, right: None})),
                Some(ref mut node) => node.insert(value)
            }
        }
    }

    pub fn lookup(&self, value: i32) -> bool {
        if self.value == value {
            return true;
        }
        if self.value > value {
            match self.left {
                None => false,
                Some(ref node) => node.lookup(value),
            }
        } else {
            match self.right {
                None => false,
                Some(ref node) => node.lookup(value)
            }
        }
    }

    // pub fn is_empty(&self) -> bool {
        
    // }
}


#[cfg(test)]

mod tests {
    use crate::binary_search_tree::*;

    #[test]
    fn test_insert() {
        let mut tree = Node::new(0);
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree.lookup(3), true);
        assert_eq!(tree.lookup(4), false);
    }
}





