fn main() {
    println!("Hello, world! 3");
}

#[cfg(test)]
pub mod test {
    #[test]
    fn test_add() {
        assert_eq!(1 + 1, 2)
    }
}
