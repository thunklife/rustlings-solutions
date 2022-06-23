// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

// Original
// fn main() {
//     let number = "T-H-R-E-E"; // don't change this line
//     println!("Spell a Number : {}", number);
//     number = 3;
//     println!("Number plus two is : {}", number + 2);
// }

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // Shadowing
    let number = 3;
    println!("Number plus two is : {}", number + 2);
}