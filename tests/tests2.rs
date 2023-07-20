// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

// ORIGINAL
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn you_can_assert_eq() {
//         assert_eq!();
//     }
// }

// UPDATED
#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!("this", "this");
    }
}