use crate::solution::Solution;

/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let length = chars.len();
        let mut result = 0;

        let get_pre_symbol = |index: usize| {
            if index == 0 {
                None
            } else {
                chars.get(index - 1)
            }
        };

        for index in 0..length {
            let symbol = &chars[index];

            match symbol {
                'I' => result += 1,
                'V' => {
                    if let Some(&'I') = get_pre_symbol(index) {
                        result += 3;
                    } else {
                        result += 5;
                    }
                }
                'X' => {
                    if let Some(&'I') = get_pre_symbol(index) {
                        result += 8;
                    } else {
                        result += 10
                    }
                }
                'L' => {
                    if let Some(&'X') = get_pre_symbol(index) {
                        result += 30;
                    } else {
                        result += 50
                    }
                }
                'C' => {
                    if let Some(&'X') = get_pre_symbol(index) {
                        result += 80;
                    } else {
                        result += 100
                    }
                }
                'D' => {
                    if let Some(&'C') = get_pre_symbol(index) {
                        result += 300;
                    } else {
                        result += 500
                    }
                }
                'M' => {
                    if let Some(&'C') = get_pre_symbol(index) {
                        result += 800;
                    } else {
                        result += 1000
                    }
                }
                _ => panic!("unknown symbol"),
            }
        }
        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn test_1() {
        let input = "III".to_string();
        let result = Solution::roman_to_int(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let input = "LVIII".to_string();
        let result = Solution::roman_to_int(input);
        assert_eq!(result, 58);
    }

    #[test]
    fn test_3() {
        let input = "MCMXCIV".to_string();
        let result = Solution::roman_to_int(input);
        assert_eq!(result, 1994);
    }
}
