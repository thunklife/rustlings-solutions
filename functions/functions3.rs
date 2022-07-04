// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

// Original
// fn main() {
//     call_me();
// }

// fn call_me(num: u32) {
//     for i in 0..num {
//         println!("Ring! Call number {}", i + 1);
//     }
// }

// Updated
fn main() {
    call_me(12);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}