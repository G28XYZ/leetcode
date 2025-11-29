use std::cmp::max;

fn max_sub_array(nums: Vec<i32>) -> i32 {
	let mut max_sum = nums[0];
	let mut cur_sum = nums[0];

	for &i in &nums[1..] {
		cur_sum = max(i, cur_sum + i);
		max_sum = max(max_sum, cur_sum);
	}

	max_sum
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn max_sub_array_test_1() {
			let arr = vec![-2,1,-3,4,-1,2,1,-5,4];
	
			let res = max_sub_array(arr);
			print!("{:?}", res);
			assert_eq!(res, 6);
	}

}