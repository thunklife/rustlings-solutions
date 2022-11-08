// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

// ORIGINAL
// fn main() {
//     let answer = current_favorite_color();
//     println!("My current favorite color is {}", answer);
// }

// fn current_favorite_color() -> String {
//     "blue"
// }

// UPDATED
fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    String::from("blue")
}