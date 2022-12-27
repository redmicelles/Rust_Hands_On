#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x:i32, y:i32 },
    Write(String),
    ChnageColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //method body
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    //m.call();

    println!("{:#?}", m)
}
