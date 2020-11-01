fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32},
        Write(String),
        ChangeColor(i32,i32,i32),
    }
    #[derive(Debug)]
    enum IpAddr{
        V4(String),
        V6(String),
    }
    
    impl Message {
        fn call(&self) {
    
        }
    }
    let home = IpAddr::V4(String::from("192.168.0.3"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Home address is {:?} and loopback {:?}", home, loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_numer: Option<i32> = None;
}
