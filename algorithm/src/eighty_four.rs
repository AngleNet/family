use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut ma = vec![vec![0_i32; heights.len()]; heights.len()];
        for i in 0..heights.len() {
            for j in i..heights.len() {
                if i == j {
                    ma[i][j] = heights[i];
                } else {
                    ma[i][j] = cmp::min(ma[i][j - 1], heights[j]);
                }
            }
        }
        let mut a = heights[0];
        for j in 1..heights.len() {
            let mut k: i32 = j as i32;
            while k >= 0 {
                a = cmp::max(a, ma[k as usize][j] * (j as i32 + 1 - k as i32));
                k -= 1;
            }
        }
        return a;
    }
}

#[cfg(test)]
mod test {
    use crate::eighty_four::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }
}
