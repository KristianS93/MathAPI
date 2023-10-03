pub fn subtract(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = subtract(4, 2);
        assert_eq!(result, 2);
    }
}
