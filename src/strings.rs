fn main() {
    // let mut s = String::new();
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + &s2 + &s3;
    // println!("{s}, {s2}{s3}");

    // let s = format!("{}-{}-{}", s1, s2, s3);
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // let s1 = "Hello.";
    // let H = s1[0];

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    for c in hello.chars() {
        println!("{}", c);
    }

    println!("---");
    for c in hello.bytes() {
        println!("{c}");
    }
}
