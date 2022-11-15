pub mod alternating_characters {

    pub const MAX_LENGHT: usize = 100_000;

    pub fn alternating_characters(s: &str) -> u32 {
        if s.len() > MAX_LENGHT {
            panic!("string too long");
        }

        if s.len() == 0 {
            panic!("string too short");
        }

        let mut current_char = ' ';
        let mut deletions = 0;
        for (_index, char) in s.chars().enumerate() {
            if char == current_char {
                deletions += 1;
            }
            current_char = char;
        }

        deletions
    }
}

#[cfg(test)]
mod tests {
    use crate::alternating_characters::alternating_characters::*;

    #[test]
    #[should_panic(expected = "string too long")]
    fn text_too_long() {
        assert_eq!(alternating_characters(&"A".repeat(MAX_LENGHT + 1)), 10);
    }

    #[test]
    #[should_panic(expected = "string too short")]
    fn text_too_short() {
        assert_eq!(alternating_characters(""), 10);
    }

    #[test]
    fn maxiimum_text_lenght() {
        assert_eq!(alternating_characters(&"A".repeat(MAX_LENGHT)), 99_999);
    }

    #[test]
    fn test_00_1() {
        assert_eq!(alternating_characters("AAAA"), 3)
    }

    #[test]
    fn test_00_2() {
        assert_eq!(alternating_characters("BBBBB"), 4)
    }

    #[test]
    fn test_00_3() {
        assert_eq!(alternating_characters("ABABABAB"), 0)
    }

    #[test]
    fn test_00_4() {
        assert_eq!(alternating_characters("BABABA"), 0)
    }

    #[test]
    fn test_00_5() {
        assert_eq!(alternating_characters("AAABBB"), 4)
    }

    #[test]
    fn test_13_1() {
        assert_eq!(alternating_characters("AAABBBAABB"), 6)
    }

    #[test]
    fn test_13_2() {
        assert_eq!(alternating_characters("AABBAABB"), 4)
    }

    #[test]
    fn test_13_3() {
        assert_eq!(alternating_characters("ABABABAA"), 1)
    }

    #[test]
    fn test_14_1() {
        assert_eq!(alternating_characters("ABBABBAA"), 3)
    }
}
