// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

// ORIGINAL
// fn main() {
//     let mut shopping_list: Vec<?> = Vec::new();
//     shopping_list.push("milk");
// }

// UPDATED
fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}