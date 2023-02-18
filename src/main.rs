fn main() {
    let some_number = Some(5);
    let some_string = Some("a String");

    let absent_number: Option<i32> = None;

    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);

    // #[derive(Debug)]
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // impl Message {
    //     fn call(&self) {
    //         // method body would be defined here
    //         // メソッド本体はここに定義される
    //     }
    // }

    // let m = Message::Write(String::from("hello"));

    // println!("{:#?}", m::Write);

    // #[derive(Debug)]
    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }

    // let home = IpAddr::V4(127, 0, 0, 1);
    // println!("{:#?}", home);

    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
}
