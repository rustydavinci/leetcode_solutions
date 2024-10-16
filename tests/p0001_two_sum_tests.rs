#[cfg(test)]
mod p0001_two_sum_tests {
    use leetcode_solutions::Solution;

    #[test]
    fn test_case_01() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];

        let result = Solution::two_sum(nums, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_02() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expected = vec![1, 2];

        let result = Solution::two_sum(nums, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_03() {
        let nums = vec![3, 3];
        let target = 6;
        let expected = vec![0, 1];

        let result = Solution::two_sum(nums, target);
        assert_eq!(result, expected);
    }
}
