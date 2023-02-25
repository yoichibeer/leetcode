pub struct Solution {}
impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort();
        println!("{:?}", nums);
        let mut left_idx = 0;
        let mut answer = 0;
        while left_idx < nums.len() {
            let min = lower - nums[left_idx];
            let max = upper - nums[left_idx];
            let mut right_min_idx = nums.partition_point(|&x| x < min);
            if right_min_idx <= left_idx {
                right_min_idx = left_idx + 1;
            }
            let mut right_max_idx = nums.partition_point(|&x| x <= max);
            if right_max_idx <= left_idx {
                right_max_idx = left_idx + 1;
            }
            println!(
                "left_idx={},num={},idx={},idx={},min={},max={}",
                left_idx, nums[left_idx], right_min_idx, right_max_idx, min, max
            );
            println!("ans={}", (right_max_idx - right_min_idx));
            answer += (right_max_idx - right_min_idx) as i64;
            left_idx += 1;
        }
        answer
    }
}

#[cfg(test)]
mod leetcode6355 {
    use crate::Solution;

    #[test]
    fn test1() {
        assert_eq!(6, Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6));
    }

    #[test]
    fn test2() {
        assert_eq!(1, Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11));
    }

    #[test]
    fn test3() {
        assert_eq!(15, Solution::count_fair_pairs(vec![0, 0, 0, 0, 0, 0], 0, 0));
    }

    #[test]
    fn test4() {
        assert_eq!(
            15,
            Solution::count_fair_pairs(vec![0, 0, 0, 0, 0, 0], -1000000000, 1000000000)
        );
    }
}
