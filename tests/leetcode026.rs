pub struct Solution {}

impl Solution {
    // pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    //     let mut count = 0;
    //     let mut prev = 101;
    //     for i in 0..nums.len() {
    //         let num = nums[i];
    //         if num != prev {
    //             nums[count] = num;
    //             prev = num;
    //             count += 1;
    //         }
    //     }
    //     count as i32
    // }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod leetcode026 {
    use crate::Solution;

    #[test]
    fn test0() {
        let mut nums: Vec<i32> = vec![1, 1, 2];
        let expected_k = 2;
        let actual_k = Solution::remove_duplicates(&mut nums);
        println!("{:?}", nums);
        assert_eq!(expected_k, actual_k);
    }
}
