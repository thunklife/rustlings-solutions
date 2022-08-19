// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

// ORIGINAL
// fn main() {
//     let a = ???

//     if a.len() >= 100 {
//         println!("Wow, that's a big array!");
//     } else {
//         println!("Meh, I eat arrays like that for breakfast.");
//     }
// }

// UPDATED
fn main() {
    // Arrays are fixed size every element must be initialized to a valid value when the array is initialized.
    let a = [1; 1000];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}