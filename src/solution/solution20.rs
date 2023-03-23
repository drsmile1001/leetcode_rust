use crate::solution::Solution;

/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut chars = s.chars();

        Self::closed_by(&mut chars, None)
    }

    fn closed_by(chars: &mut std::str::Chars, closed_by_char: core::option::Option<char>) -> bool {
        let next_char = chars.next();

        if next_char == closed_by_char {
            return true;
        }

        match next_char {
            Some('(') => {
                Self::closed_by(chars, Some(')')) && Self::closed_by(chars, closed_by_char)
            }
            Some('[') => {
                Self::closed_by(chars, Some(']')) && Self::closed_by(chars, closed_by_char)
            }
            Some('{') => {
                Self::closed_by(chars, Some('}')) && Self::closed_by(chars, closed_by_char)
            }
            _ => false,
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn test_1() {
        let input = "()".to_string();
        let result = Solution::is_valid(input);

        assert_eq!(result, true)
    }

    #[test]
    fn test_2() {
        let input = "()[]{}".to_string();
        let result = Solution::is_valid(input);

        assert_eq!(result, true)
    }

    #[test]
    fn test_3() {
        let input = "(]".to_string();
        let result = Solution::is_valid(input);

        assert_eq!(result, false)
    }

    #[test]
    fn test_4() {
        let input = "{[]}".to_string();
        let result = Solution::is_valid(input);

        assert_eq!(result, true)
    }

    #[test]
    fn test_5() {
        let input = "(){}}{".to_string();
        let result = Solution::is_valid(input);

        assert_eq!(result, false)
    }
}
