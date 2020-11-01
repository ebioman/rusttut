fn main() {
    let mut v = Vec::new();
    let v2 = vec![1,2,3,4,5];
    v.push(5);
    v.push(4);
    v.push(5);
    v.push(9);

    match v.get(2) {
        Some(third) => println!("The third element is {}",third),
        None => println!("There is no third element!"),
    }

    for i in &mut v {
        println!("element is {}",i);
        *i += 50;
        println!("element is {}",i);
    }

    enum Excel {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Excel::Int(3),
        Excel::Text(String::from("Blue")),
        Excel::Float(10.11515),
    ];
}
