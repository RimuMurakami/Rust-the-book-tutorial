fn vec() {
    // let v: Vec<i32> = Vec::new();
    // let mut v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);

    let third = &v[2];
    println!("The third element is {third}");

    match v.get(4) {
        Some(num) => println!("is {num}."),
        None => println!("is no."),
    }

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);

    let v = vec![100, 32, 45];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![32, 43, 54];
    for i in &mut v {
        *i += 100;
        println!("{i}");
    }
    println!("{:?}", v);
}
