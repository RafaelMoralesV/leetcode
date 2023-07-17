impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        use std::convert::TryInto;

        for sol in 0..nums.len() + 1 {
            let count = nums.iter().fold(0, |acc, e| {
                if *e as usize >= sol.try_into().unwrap() {
                    acc + 1
                } else {
                    acc
                }
            });

            if count == sol {
                return sol.try_into().unwrap();
            }
        }

        -1
    }
}

fn main() {
    todo!();
}
