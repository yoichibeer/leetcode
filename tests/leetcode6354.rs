pub struct Solution {}
impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut l_idx: i32 = 0;
        let mut r_idx: i32 = nums.len() as i32 - 1;
        let mut answer: i64 = 0;
        while l_idx <= r_idx {
            if l_idx == r_idx {
                answer += nums[l_idx as usize] as i64;
            } else {
                let l_i32 = nums[l_idx as usize];
                let r_i32 = nums[r_idx as usize];
                answer += format!("{}{}", l_i32, r_i32).parse::<i64>().unwrap();
            }
            l_idx += 1;
            r_idx -= 1;
        }
        answer
    }
}

#[cfg(test)]
mod leetcode6354 {
    use crate::Solution;

    #[test]
    fn test1() {
        assert_eq!(4, Solution::find_the_array_conc_val(vec![4]));
    }

    #[test]
    fn test4() {
        assert_eq!(
            74 + 522,
            Solution::find_the_array_conc_val(vec![7, 52, 2, 4])
        );
    }
}
