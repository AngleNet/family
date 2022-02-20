struct Solution34;

impl Solution34 {
    fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (s, e) = Solution34::search_range_rec(&nums, 0, nums.len() as i32 - 1, target);
        return vec![s, e];
    }

    fn search_range_rec(nums: &Vec<i32>, i: i32, j: i32, target: i32) -> (i32, i32) {
        if i > j || target < nums[i as usize] || target > nums[j as usize] {
            return (-1, -1);
        }
        if j == i && nums[i as usize] == target {
            return (i, i);
        }
        let k = (i + j) / 2;
        if nums[k as usize] < target {
            return Solution34::search_range_rec(nums, k + 1, j, target);
        } else if nums[k as usize] > target {
            return Solution34::search_range_rec(nums, i, k - 1, target);
        }
        let (ls, _le) = Solution34::search_range_rec(nums, i, k - 1, target);
        let (rs, re) = Solution34::search_range_rec(nums, k + 1, j, target);
        if ls == -1 && rs == -1 {
            return (k, k);
        } else if ls == -1 {
            return (k, re);
        } else if rs == -1 {
            return (ls, k);
        } else {
            return (ls, re);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tenth::Solution34;

    #[test]
    fn test_34() {
        assert_eq!(vec![0, 0], Solution34::search_range(vec![1, 3], 1));
        assert_eq!(
            vec![3, 4],
            Solution34::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        )
    }
}
