fn main() {
    println!("Hello w💩rld !");
    println!("Je sais compter 2 + 2 = {} ", add(2, 2))
}

fn add(a: u8, b: u8) -> u8 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn test_add() {
        assert_eq!(add(42, 24), 66);
    }
}
