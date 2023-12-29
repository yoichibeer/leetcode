pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.truncate(m as usize);
        nums1.push(1000000001);
        let mut idx1 = 0;
        let mut idx2 = 0;
        loop {
            if idx2 == n as usize {
                nums1.pop();
                break;
            }
            if nums1[idx1] > nums2[idx2] {
                nums1.insert(idx1, nums2[idx2]);
                idx2 += 1;
            }
            idx1 += 1;
        }
    }
    // pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    //     let mut idx1 = 0;
    //     let mut idx2 = 0;
    //     loop {
    //         if idx2 == n as usize {
    //             nums1.truncate((m + n) as usize);
    //             break;
    //         }
    //         if nums1[idx1] > nums2[idx2] || idx1 == idx2 + m as usize {
    //             nums1.insert(idx1, nums2[idx2]);
    //             idx2 += 1;
    //         }
    //         idx1 += 1;
    //     }
    // }
}

#[cfg(test)]
mod leetcode088 {
    use crate::Solution;

    #[test]
    fn test0() {
        let mut nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
        let mut nums2: Vec<i32> = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        println!("{:?}", nums1);
        assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
    }
}
