use std::collections::HashMap;

///Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
///You may assume that each input would have exactly one solution, and you may not use the same element twice.
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements: HashMap<i32, i32> = HashMap::new(); 

    for i in 0..nums.len() {
        let complement_index = complements.get(&nums[i]);
        if let Some(idx) = complement_index {
            return vec![i as i32, *idx];
        } 
        complements.insert((target - nums[i]) as i32, i as i32);
    }
    nums
}

#[test]
fn two_sum_solution_test() {
    assert_eq!(vec![2,1], two_sum(vec![1,2,5,10], 7));
}