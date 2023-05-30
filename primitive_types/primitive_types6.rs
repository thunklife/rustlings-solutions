// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put this right into the `println!` where the ??? is.
// Execute `rustlings hint primitive_types6` for hints!

// ORIGINAL
// fn main() {
//     let numbers = (1, 2, 3);
//     println!("The second number is {}", ???);
// }

// UPDATED
fn main() {
    let numbers = (1, 2, 3);
    println!("The second number is {}", numbers.1);
}