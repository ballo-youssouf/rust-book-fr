#![allow(unused_variables)]
fn main() {}

// ANCHOR: here
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("deux plus deux ne vaut pas quatre"))
        }
    }
}
// ANCHOR_END: here
