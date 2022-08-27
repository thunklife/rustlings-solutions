// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

// ORIGINAL
// #[derive(Debug)]
// enum Message {
//     // TODO: define a few types of messages as used below
// }

// fn main() {
//     println!("{:?}", Message::Quit);
//     println!("{:?}", Message::Echo);
//     println!("{:?}", Message::Move);
//     println!("{:?}", Message::ChangeColor);
// }

// UPDATED
#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    ChangeColor,
    Move
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}