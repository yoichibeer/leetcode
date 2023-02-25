pub struct Solution {}
impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut answer = 1;
        let mut n = n;
        loop {
            let mut i = 0;
            let mut prev = 0;
            loop {
                let pow = 2_i32.pow(i);
                if pow == n {
                    return answer;
                }
                if pow > n {
                    n = std::cmp::min(n - prev, pow - n);
                    answer += 1;
                    break;
                }
                i += 1;
                prev = pow;
            }
        }
    }
}

#[cfg(test)]
mod leetcode6365 {
    use crate::Solution;

    #[test]
    fn test1() {
        assert_eq!(1, Solution::min_operations(1));
    }
    #[test]
    fn test2() {
        assert_eq!(1, Solution::min_operations(2));
    }
    #[test]
    fn test3() {
        assert_eq!(2, Solution::min_operations(3));
    }
    #[test]
    fn test4() {
        assert_eq!(1, Solution::min_operations(4));
    }
    #[test]
    fn test5() {
        assert_eq!(2, Solution::min_operations(5));
    }
    #[test]
    fn test39() {
        assert_eq!(3, Solution::min_operations(39));
    }
    #[test]
    fn test54() {
        assert_eq!(3, Solution::min_operations(54));
    }
}
