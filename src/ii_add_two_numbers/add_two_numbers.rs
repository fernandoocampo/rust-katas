// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
// while let Some(ref mut x)=self.next  {
//     self=x;
// }
// impl fmt::Display for ListNode {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.val);
//         while Some(ref value) = node {

//         }
//     }
// }

impl AsRef<ListNode> for ListNode {
    fn as_ref(&self) -> &ListNode {
        &self
    }
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn new_from_values(values: Vec<i32>) -> Self {
        if values.len() == 0 {
            return ListNode::new(0);
        }

        let mut root = ListNode::new(values[0]);

        // let node: &mut ListNode = &mut root;
        // for i in 1..values.len() {
        //     let new_node = ListNode::new(values[i]);
        //     node.next = Some(Box::new(new_node));
        //     node = node.next;
        // }

        root
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode::new(2)))
}
