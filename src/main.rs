use std::io;
fn main() {
    // let x = (500, 6.4, 1);

    // let five_hundred = x.0;

    // println!("{}", five_hundred);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered wad not a number.");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
