use crate::solution::Solution;

/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */
// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let number_chars = x.to_string().chars().collect::<Vec<char>>();
        let number_count = number_chars.len();
        let half = number_count / 2;
        for index in 0..half {
            if number_chars[index] != number_chars[number_count - index - 1] {
                return false;
            }
        }
        true
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn test_1() {
        let input = 121;
        let result = Solution::is_palindrome(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let input = -121;
        let result = Solution::is_palindrome(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let input = 10;
        let result = Solution::is_palindrome(input);
        assert_eq!(result, false);
    }
}
