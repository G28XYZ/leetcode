pub fn length_of_longest_substring(s: String) -> i32 {
	let mut res = "".to_string();
	let mut count: i32 = 0;

	for c in s.split("") {
		if res.contains(c) {
			let idx = res.find(c).unwrap();
			if res.len() > 1 {
				res = format!("{}{}", &res[idx+1..], c);
			} else {
				res = c.to_string();
			}
		} else {
			res = format!("{}{}", res, c);
			if res.len() as i32 > count {
				count = res.len() as i32;
			}
		}
		dbg!(&res);
	}

	count
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn length_of_longest_substring_test_1() {
		let res = length_of_longest_substring("dvdf".to_string());

		assert_eq!(res, 3);
	}

	#[test]
	fn length_of_longest_substring_test_2() {
		let res = length_of_longest_substring("ohvhjdml".to_string());

		assert_eq!(res, 6);
	}
}