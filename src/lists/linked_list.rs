#[derive(Clone)]
struct Node<T: Clone> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
}

impl<T: Clone> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn create_box(&self, value: T) -> Box<Node<T>> {
        Box::new(Node {
            value: value,
            next: None,
        })
    }

    fn push(&mut self, data: T) {
        let nn = Box::new(Node {
            value: data,
            next: self.head.take(),
        });
        self.head = Some(nn);
    }

    fn push_right(&mut self, data: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node {
                value: data,
                next: None,
            }));
            return;
        }

        let mut node = self.head.as_mut();
        loop {
            if node.as_ref().unwrap().next.is_none() {
                break;
            }

            node = node.unwrap().next.as_mut();
        }

        node.unwrap().as_mut().next = Some(Box::new(Node {
            value: data,
            next: None,
        }));
    }

    fn collect(&self) -> Vec<T> {
        let mut out = Vec::<T>::new();

        let mut cur = self.head.clone();
        while let Some(ref n) = cur {
            out.push(n.value.clone());
            cur = n.next.clone();
        }

        out
    }
}

#[cfg(test)]
#[test]
fn test_linked_list_push_left() {
    let mut list = LinkedList::<i32>::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    assert_eq!(list.collect(), vec![4, 3, 2, 1]);
}

#[test]
fn test_linked_list_push_right() {
    let mut list = LinkedList::<i32>::new();
    list.push_right(1);
    list.push_right(2);
    list.push_right(3);
    list.push_right(4);
    assert_eq!(list.collect(), vec![1, 2, 3, 4]);
}
