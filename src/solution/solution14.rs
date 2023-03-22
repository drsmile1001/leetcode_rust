use crate::solution::Solution;

/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].chars();
        let mut next_prefix: String;
        for next_word in &strs[1..] {
            let mut char_index: usize = 0;
            next_prefix = next_word
                .chars()
                .take_while(|c| {
                    let m = prefix.next();
                    char_index += 1;
                    if m == None {
                        return false;
                    }
                    return m.unwrap() == *c;
                })
                .collect::<String>();
            prefix = next_prefix.chars();
        }

        prefix.collect()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn test_1() {
        let input = [
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]
        .to_vec();

        let result = Solution::longest_common_prefix(input);

        assert_eq!(result, "fl".to_string());
    }

    #[test]
    fn test_2() {
        let input = ["dog".to_string(), "racecar".to_string(), "car".to_string()].to_vec();

        let result = Solution::longest_common_prefix(input);

        assert_eq!(result, "".to_string());
    }
}
