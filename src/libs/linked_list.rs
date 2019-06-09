// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn vec_to_list_node(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(-1)));
    let mut copy = &mut dummy;
    for &val in vec.iter() {
        copy.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
        copy = &mut copy.as_mut().unwrap().next;
    }
    dummy.unwrap().next
}
