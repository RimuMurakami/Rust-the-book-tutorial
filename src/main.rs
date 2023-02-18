fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("the first word is: {}", word);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);
    println!("the first word is: {}", word);

    let word = first_word(my_string_literal);
    println!("the first word is: {}", word);

    // s.clear();
    // println!("{}", s);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    // for element in &slice {
    //     println!("{}", element);
    // }
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }
