// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

// ORIGINAL
// mod sausage_factory {
//     fn make_sausage() {
//         println!("sausage!");
//     }
// }

// fn main() {
//     sausage_factory::make_sausage();
// }

// UPDATED
mod sausage_factory {
    pub fn make_sausage() {
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
