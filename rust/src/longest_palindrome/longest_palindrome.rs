pub fn longest_palindrome(s: String) -> String {
    // Тривиальные случаи: пустая строка или длина 1 уже палиндром.
    if s.len() < 2 {
        return s;
    }

    let chars: Vec<char> = s.chars().collect();
    let n: usize = chars.len();
    let mut best_start = 0;
    let mut best_len = 1;

    // Расширяемся от центра (один символ или между символами) и обновляем лучший результат.
    let expand = |mut l: isize, mut r: isize, best_start: &mut usize, best_len: &mut usize| {
        while l >= 0 && (r as usize) < n && chars[l as usize] == chars[r as usize] {
            l -= 1;
            r += 1;
        }
        let start = (l + 1) as usize;
        let len = (r - l - 1) as usize;
        if len > *best_len {
            *best_start = start;
            *best_len = len;
        }
    };

    for i in 0..n {
        // Центр в одном символе (нечетная длина).
        expand(i as isize, i as isize, &mut best_start, &mut best_len);
        // Центр между символами (четная длина).
        expand(i as isize, i as isize + 1, &mut best_start, &mut best_len);
    }

    chars[best_start..best_start + best_len].iter().collect()
}


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn longest_palindrome_test_1() {
		let res = longest_palindrome("babad".to_string());

		assert!(["bab", "aba"].contains(&res.as_str()));
	}

	#[test]
	fn longest_palindrome_test_2() {
		let res = longest_palindrome("cbbd".to_string());

		assert_eq!(res, "bb".to_string());
	}

	#[test]
	fn longest_palindrome_test_3() {
		let res = longest_palindrome("abbcccba".to_string());

		assert_eq!(res, "bcccb".to_string());
	}

	#[test]
	fn longest_palindrome_test_4() {
		let res = longest_palindrome("ccd".to_string());

		assert_eq!(res, "cc".to_string());
	}

	#[test]
	fn longest_palindrome_test_5() {
		let res = longest_palindrome("ac".to_string());

		assert_eq!(res, "a".to_string());
	}
	
}
