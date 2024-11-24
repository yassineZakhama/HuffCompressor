use std::collections::HashMap;

pub fn calculate_frequency_table(s: &str) -> HashMap<char, i32> {
    let mut result = HashMap::new();

    for c in s.chars() {
        let count = result.entry(c).or_insert(0);
        *count += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_frequency() {
        const INPUT: &str = "Hello, world!";

        let result = calculate_frequency_table(INPUT);

        assert_eq!(result.len(), 10);

        assert_eq!(result.get(&'H').unwrap(), &1);
        assert_eq!(result.get(&'e').unwrap(), &1);
        assert_eq!(result.get(&'l').unwrap(), &3);
        assert_eq!(result.get(&'o').unwrap(), &2);
        assert_eq!(result.get(&',').unwrap(), &1);
        assert_eq!(result.get(&' ').unwrap(), &1);
        assert_eq!(result.get(&'w').unwrap(), &1);
        assert_eq!(result.get(&'r').unwrap(), &1);
        assert_eq!(result.get(&'d').unwrap(), &1);
        assert_eq!(result.get(&'!').unwrap(), &1);
    }
}
