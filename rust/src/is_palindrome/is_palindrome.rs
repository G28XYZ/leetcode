pub fn is_palindrome(x: i32) -> bool {
	let x_str = x.to_string();
	let reversed: String = x_str.chars().rev().collect();

	x_str == reversed
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn is_palindrome_test_1() {
		let res = is_palindrome(121);

		assert_eq!(res, true);
	}

	#[test]
	fn is_palindrome_test_2() {
		let res = is_palindrome(123);

		assert_eq!(res, false);
	}

	#[test]
	fn is_palindrome_test_3() {
		let res = is_palindrome(1000021);

		assert_eq!(res, false);
	}

}