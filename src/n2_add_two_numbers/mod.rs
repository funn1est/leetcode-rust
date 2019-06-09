use super::libs::linked_list::{vec_to_list_node, ListNode};

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy = Some(Box::new(ListNode::new(-1)));
        let mut copy = &mut dummy;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() {
            let v1 = if let Some(node) = l1 {
                l1 = node.next;
                node.val
            } else {
                0
            };
            let v2 = if let Some(node) = l2 {
                l2 = node.next;
                node.val
            } else {
                0
            };
            let mut sum = v1 + v2 + carry;
            carry = sum / 10;
            sum = sum % 10;
            copy.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            copy = &mut copy.as_mut().unwrap().next;
        }
        if carry != 0 {
            copy.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::vec_to_list_node;
    use super::Solution;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            Solution::add_two_numbers(
                vec_to_list_node(vec![2, 4, 3]),
                vec_to_list_node(vec![5, 6, 4])
            ),
            vec_to_list_node(vec![7, 0, 8])
        );
        assert_eq!(
            Solution::add_two_numbers(
                vec_to_list_node(vec![2, 4, 6]),
                vec_to_list_node(vec![5, 5, 4])
            ),
            vec_to_list_node(vec![7, 9, 0, 1])
        );
        assert_eq!(
            Solution::add_two_numbers(vec_to_list_node(vec![1, 8]), vec_to_list_node(vec![0])),
            vec_to_list_node(vec![1, 8])
        );
    }
}
