fn main() {
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

    strings.append(vec!('e', 'f', 'g'));

    println!("{:?}", strings);


    // let numbers: [i32; 5];

    // numbers = [1, 1, 1, 1, 1];

    // println!("{:?}", numbers); 
}
