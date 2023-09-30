#[warn(unused_imports)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new() -> Self {
        ListNode { next: None, val: 0 }
    }

    fn new_with_val(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn get_number_from_list(
    left: Option<Box<ListNode>>,
    right: Option<Box<ListNode>>,
    value: i32,
) -> Option<Box<ListNode>> {
    if left.is_none() && right.is_none() && value == 0 {
        return None;
    }

    let (lv, ln) = if let Some(l) = left {
        (l.val, l.next)
    } else {
        (0, None)
    };
    let (rv, rn) = if let Some(r) = right {
        (r.val, r.next)
    } else {
        (0, None)
    };
    let val = lv + rv + value;
    Some(Box::new(ListNode {
        val: val % 10,
        next: get_number_from_list(ln, rn, val / 10),
    }))
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() {
        return None;
    }

    get_number_from_list(l1, l2, 0)
}

#[cfg(test)]
mod add_two_numbers_tests {
    // use crate::ii_add_two_numbers::add_two_numbers::{add_two_numbers, ListNode};

    // #[test]
    // fn test_add_two_numbers() {
    //   // Given
    //   let left = build_tree(vec![2,4,3]);
    //   let right = build_tree(vec![5,6,4]);
    // }

    // fn build_tree(values: Vec::<i32>) -> Option<Box<ListNode>> {
    //   let mut node = Some(Box::new(ListNode::new(values[0])));
    //   let root = node.as_ref();
    //   for &value in values[1..].into_iter() {
    //     node.unwrap().next = Some(Box::new(ListNode::new(value)));
    //     node = node.unwrap().next;
    //   }

    //   Some(root.unwrap().to_owned())
    // }
}
