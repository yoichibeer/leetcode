pub struct Solution {}

impl Solution {
    // pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    //     nums.retain(|&x| x != val);
    //     nums.len() as i32
    // }
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            let num = nums[i];
            if num != val {
                nums[count] = num;
                count += 1;
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod leetcode027 {
    use crate::Solution;

    #[test]
    fn test0() {
        let mut nums: Vec<i32> = vec![3, 2, 2, 3];
        let expected_k = 2;
        let val = 3;
        let actual_k = Solution::remove_element(&mut nums, val);
        println!("{:?}", nums);
        assert_eq!(expected_k, actual_k);
    }
}
