// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

// ORIGINAL
// #[derive(Debug)]
// enum Message {
//     // TODO: define the different variants used below
// }

// impl Message {
//     fn call(&self) {
//         println!("{:?}", &self);
//     }
// }

// fn main() {
//     let messages = [
//         Message::Move { x: 10, y: 30 },
//         Message::Echo(String::from("hello world")),
//         Message::ChangeColor(200, 255, 255),
//         Message::Quit,
//     ];

//     for message in &messages {
//         message.call();
//     }
// }

// UPDATED
#[derive(Debug)]
enum Message {
    Move{x: i32, y: i32},
    Echo(String),
    ChangeColor(u16, u16, u16),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}