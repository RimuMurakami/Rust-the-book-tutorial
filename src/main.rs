// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;

//     println!("{}, {}", r1, r2);
// }

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}
