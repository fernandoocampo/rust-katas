use std::ops::Deref;

// Definition for singly-linked list.
// https://betterprogramming.pub/learning-rust-building-a-linked-list-102bcb08f93b
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<'a, T> {
    pub val: T,
    // the passed box will live as long as the parent Linked List.
    // This allows the compiler to check that all references will
    // be valid.
    pub next: Option<&'a Box<ListNode<'a, T>>>,
}

impl<'a> ListNode<'a, i32> {
    #[inline]
    fn new() -> Self {
        ListNode { next: None, val: 0 }
    }

    fn new_with_val(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn create_box(&self, x: i32) -> Box<ListNode<'a, i32>> {
        Box::new(ListNode { val: x, next: None })
    }

    fn push_left(&mut self, node: &'a mut Box<ListNode<'a, i32>>) {
        if self.next.is_none() {
            self.next = Some(node);
            return;
        };

        node.next = self.next;
        self.next = Some(node);
    }

    // fn push_right(mut self, node: &'a mut Box<ListNode<'a, i32>>) {
    //     if self.next.is_none() {
    //         self.next = Some(node);
    //         return;
    //     };

    //     let mut thenode = self.next.unwrap().as_ref();

    //     thenode.push_right(node);
    // }

    fn collect(&self) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        if self.next.is_none() {
            return result;
        }

        let mut node = self.next.unwrap();

        result.push(node.val);
        while node.next.is_some() {
            node = node.next.unwrap();
            result.push(node.val);
        }

        result
    }
}

#[cfg(test)]
mod add_two_numbers_tests {
    use crate::lists::linked_node::ListNode;

    #[test]
    fn test_list_push_left() {
        let mut list: ListNode<i32> = ListNode::<i32>::new();
        let mut box_ = list.create_box(1);
        let mut box_2 = list.create_box(2);
        let mut box_3 = list.create_box(3);
        let mut box_4 = list.create_box(4);

        list.push_left(&mut box_);
        list.push_left(&mut box_2);
        list.push_left(&mut box_3);
        list.push_left(&mut box_4);

        assert_eq!(list.collect(), vec![4, 3, 2, 1]);
    }

    // #[test]
    // fn test_list_push_right() {
    //     let mut list: ListNode<i32> = ListNode::<i32>::new();
    //     let mut box_ = list.create_box(1);
    //     let mut box_2 = list.create_box(2);
    //     let mut box_3 = list.create_box(3);
    //     let mut box_4 = list.create_box(4);

    //     list.push_right(&mut box_);
    //     list.push_right(&mut box_2);
    //     list.push_right(&mut box_3);
    //     list.push_right(&mut box_4);

    //     assert_eq!(list.collect(), vec![1, 2, 3, 4]);
    // }
}
