pub mod anagram {

    use std::cmp;

    pub const MAX_LENGHT: usize = 10_000;

    pub fn anagram(s: &str) -> i32 {
        let len = s.len();

        if len > MAX_LENGHT {
            panic!("string too long");
        }

        if len == 0 {
            panic!("string too short");
        }

        if len & 1 == 1 {
            return -1;
        }

        let sublen = len / 2;

        let left = &s[..sublen];
        let right = &s[sublen..];

        let left_changes = changes(left, right);
        let right_changes = changes(right, left);

        cmp::max(left_changes, right_changes)
    }

    fn changes(first: &str, second: &str) -> i32 {
        let mut versus: String = second.to_string();

        for (_index, char) in first.chars().enumerate() {
            match versus.find(char) {
                Some(i) => {
                    versus.remove(i);
                }
                None => {}
            }
        }
        versus.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::anagram::anagram::*;

    #[test]
    #[should_panic(expected = "string too long")]
    fn text_too_long() {
        assert_eq!(anagram(&"A".repeat(MAX_LENGHT + 1)), 10);
    }

    #[test]
    #[should_panic(expected = "string too short")]
    fn text_too_short() {
        assert_eq!(anagram(""), 10);
    }

    #[test]
    fn maximum_text_lenght() {
        assert_eq!(anagram(&"A".repeat(MAX_LENGHT)), 0);
    }

    #[test]
    fn test_14_1() {
        assert_eq!(anagram("aaabbb"), 3)
    }

    #[test]
    fn test_14_2() {
        assert_eq!(anagram("ab"), 1)
    }

    #[test]
    fn test_14_3() {
        assert_eq!(anagram("abc"), -1)
    }

    #[test]
    fn test_14_4() {
        assert_eq!(anagram("mnop"), 2)
    }

    #[test]
    fn test_14_5() {
        assert_eq!(anagram("xyyx"), 0)
    }

    #[test]
    fn test_14_6() {
        assert_eq!(anagram("xaxbbbxx"), 1)
    }

    #[test]
    fn test_15_1() {
        assert_eq!(anagram("asdfjoieufoa"), 3)
    }

    #[test]
    fn test_15_2() {
        assert_eq!(anagram("fdhlvosfpafhalll"), 5)
    }

    #[test]
    fn test_15_3() {
        assert_eq!(anagram("mvdalvkiopaufl"), 5)
    }
}
