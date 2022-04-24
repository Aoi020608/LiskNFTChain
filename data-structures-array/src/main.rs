fn main() {
    /*
    // let numbers = Vec::new();
    // let strings = ['a', 'b', 'c', 'd'];
    let mut strings = vec!['a', 'b', 'c', 'd'];

    // push
    strings.push('e');

    // pop
    strings.pop();
    strings.pop();

    // insert
    strings.insert(0, 'q');

    strings.remove(0);

    // strings.append(vec!['e', 'f', 'g']);

    // println!("{:?}", strings);

    // let numbers: [i32; 5];

    // numbers = [1, 1, 1, 1, 1];

    // println!("{:?}", numbers);

    let mut my_array = MyArray::new();

    my_array.push(32);
    my_array.push(56);

    // my_array.pop();

    let new_my_array = my_array.get_array();

    println!("{:?}", new_my_array);
    */
    let word = "Hello my name is Tom";
    reverse(word);
    // println!("{:?}", reversed_word);
}

#[derive(Debug)]
struct MyArray {
    length: i32,
    data: Vec<i32>,
}

impl MyArray {
    pub fn new() -> Self {
        MyArray {
            length: 0,
            data: vec!(),
        }
    }

    pub fn get(&self, index: usize) ->  i32 {
        self.data[index]
    }

    pub fn get_array(&self) -> Vec<i32> {
        self.data.clone()
    }

    pub fn push(&mut self, item: i32) -> Vec<i32>{
        self.data.push(item);
        self.length += 1;
        self.data.clone()
    }

    pub fn pop(&mut self) {
        self.data.pop();
    }
}

// "Hello my name is Tom"
pub fn reverse(string: &str) {
    println!("{:?}", string.chars().nth(0).expect("Error"));
    // let mut iter: Vec<&str> = string.split_whitespace().collect();
    // println!("{:?}", iter.next());
    // let new_iter = iter.clone();\
    let mut new_iter: Vec<char> = Vec::new();
    let len = string.chars().count();

    for i in 1..=len {
        // new_iter.push(iter[iter.len() - i]);
        new_iter.push(string.chars().nth(len - i).expect("Error"));
    }
    let joined_word = String::from_iter(new_iter);
    println!("{:?}", joined_word);

}
