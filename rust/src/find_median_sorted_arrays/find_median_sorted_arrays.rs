pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
	let mut res = 0.0;
	
	let mut merged = nums1.clone();
	merged.extend(nums2.iter().cloned());
	merged.sort();

	if merged.len() % 2 == 0 {
		let mid_right = merged.len() / 2;
		let mid_left  = mid_right - 1;
		res = (merged[mid_left] + merged[mid_right]) as f64 / 2.0;
	} else {
		let mid: i32 = merged[merged.len() / 2];
		res = mid as f64
	}

	res
}


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn find_median_sorted_arrays_test_1() {
		let res = find_median_sorted_arrays(vec![1,3], vec![2]);

		assert_eq!(res, 2.00000);
	}

	#[test]
	fn find_median_sorted_arrays_test_2() {
		let res = find_median_sorted_arrays(vec![1,2], vec![3,4]);

		assert_eq!(res, 2.50000);
	}
}