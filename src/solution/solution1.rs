use crate::solution::Solution;

/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */
// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut first_map = std::collections::HashMap::new();
        let mut second_map = std::collections::HashMap::new();
        let number_count = nums.len();
        for index in 0..number_count {
            let number = &nums[index];
            if first_map.contains_key(number) {
                second_map.insert(number, index);
            } else {
                first_map.insert(number, index);
            }
        }

        for index in 0..number_count {
            let first_number = nums[index];
            let second_number = target - first_number;
            if first_number == second_number {
                if let Some(m) = second_map.get_key_value(&second_number) {
                    return vec![index as i32, *m.1 as i32];
                }
                continue;
            }
            if let Some(m) = first_map.get_key_value(&second_number) {
                return vec![index as i32, (*m.1) as i32];
            }
        }

        panic!("No two sum solution");
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}
