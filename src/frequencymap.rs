use std::collections::HashMap;

pub fn calculate_frequency_map(s: &str) -> HashMap<char, i32> {
    HashMap::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_frequency() {
        const INPUT: &str = "Hello, world!";

        let result = calculate_frequency_map(INPUT);

        assert_eq!(result.len(), 9);

        assert_eq!(result.get(&'H').unwrap(), &1);
        assert_eq!(result.get(&'e').unwrap(), &1);
        assert_eq!(result.get(&'l').unwrap(), &3);
        assert_eq!(result.get(&'o').unwrap(), &2);
        assert_eq!(result.get(&'w').unwrap(), &1);
        assert_eq!(result.get(&'r').unwrap(), &1);
        assert_eq!(result.get(&'d').unwrap(), &1);
        assert_eq!(result.get(&',').unwrap(), &1);
        assert_eq!(result.get(&'!').unwrap(), &1);
    }
}
