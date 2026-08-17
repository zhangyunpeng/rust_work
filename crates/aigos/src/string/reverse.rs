pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(reverse("racecar"), "racecar");
    }

    #[test]
    fn test_sentence() {
        assert_eq!(reverse("step on no pets"), "step on no pets");
    }
}
