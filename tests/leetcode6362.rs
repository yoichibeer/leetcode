pub struct Solution {}
impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut index1 = 0;
        let mut index2 = 0;
        let mut answer = vec![];
        loop {
            if index1 >= nums1.len() {
                while index2 < nums2.len() {
                    answer.push(nums2[index2].clone());
                    index2 += 1;
                }
                break;
            }
            if index2 >= nums2.len() {
                while index1 < nums1.len() {
                    answer.push(nums1[index1].clone());
                    index1 += 1;
                }
                break;
            }
            let id1 = nums1[index1][0];
            let id2 = nums2[index2][0];
            if id1 == id2 {
                answer.push(vec![id1, nums1[index1][1] + nums2[index2][1]]);
                index1 += 1;
                index2 += 1;
            }
            if id1 < id2 {
                answer.push(nums1[index1].clone());
                index1 += 1;
            }
            if id1 > id2 {
                answer.push(nums2[index2].clone());
                index2 += 1;
            }
        }
        answer
    }
}

#[cfg(test)]
mod leetcode6362 {
    use crate::Solution;

    #[test]
    fn test1() {
        let num1 = vec![vec![1, 1]];
        let num2 = vec![vec![2, 2]];
        assert_eq!(
            vec![vec![1, 1], vec![2, 2]],
            Solution::merge_arrays(num1, num2)
        );
    }
    #[test]
    fn test2() {
        let num1 = vec![vec![1, 1]];
        let num2 = vec![vec![1, 2]];
        assert_eq!(vec![vec![1, 3]], Solution::merge_arrays(num1, num2));
    }
}
