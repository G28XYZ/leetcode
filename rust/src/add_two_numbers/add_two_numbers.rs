// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut p1 = l1; // Курсор для первого списка
	let mut p2 = l2; // Курсор для второго списка
	let mut carry = 0; // Остаток между суммами цифр

	let mut head = Box::new(ListNode::new(0)); // Голова
	let mut tail = &mut head; // Хвост

	while p1.is_some() || p2.is_some() || carry > 0 {
		let mut sum = carry; // Начните с входящего остатка

		if let Some(node) = p1 {
			sum += node.val; // прибавить с первого списка
			p1 = node.next; // сдвинуть до следующего
		}

		if let Some(node) = p2 {
			sum += node.val; // прибавить со второго списка
			p2 = node.next; // сдвинуть до следующего
		}

		carry = sum / 10; // остаток если больше 9

		tail.next = Some(Box::new(ListNode::new(sum % 10))); // добавить в хвост

		tail = tail.next.as_mut().unwrap(); // сдвинуть хвост
	}

	head.next
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn add_two_numbers_test1() {
		let l1 = [9,9,9,9,9,9,9].iter().fold(None, |next, &val| {
      Some(Box::new(ListNode { val, next }))
  });
		let l2 = [9,9,9,9].iter().fold(None, |next, &val| {
      Some(Box::new(ListNode { val, next }))
  });
		let expected = [1,0,0,0,9,9,9,8].iter().fold(None, |next, &val| {
      Some(Box::new(ListNode { val, next }))
  });
		let res = add_two_numbers(l1, l2);

		assert_eq!(res, expected);
	}
}
