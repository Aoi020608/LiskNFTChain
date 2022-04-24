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
    // let word = "Hello my name is Tom";
    // reverse(word);
    // println!("{:?}", reversed_word);
    let mut arr1 = vec![3, 2, 4];
    let target = 6;
    let sum_array = two_sum(arr1, target);
    println!("{:?}", sum_array);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut pairs = Vec::new();
    for i in 0..nums.len() {
        let diff = target - nums[i];
        for j in i + 1..nums.len() {
            if diff == nums[j] {
                pairs.push(i as i32);
                pairs.push(j as i32);
                return pairs;
            }
        }
    }
    return pairs;
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
            data: vec![],
        }
    }

    pub fn get(&self, index: usize) -> i32 {
        self.data[index]
    }

    pub fn get_array(&self) -> Vec<i32> {
        self.data.clone()
    }

    pub fn push(&mut self, item: i32) -> Vec<i32> {
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

pub fn merge_sort_arrays(arr1: &mut Vec<i32>, arr2: &mut Vec<i32>) -> Vec<i32> {
    // let mut new_array: Vec<i32> = Vec::new();

    if arr1.len() == 0 {
        return arr2.to_vec();
    }

    if arr2.len() == 0 {
        return arr1.to_vec();
    }

    arr1.append(arr2);
    arr1.sort();
    println!("{:?}", arr1);

    return arr1.to_vec();
}
