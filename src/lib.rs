#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    data: i32,
    next: Option<usize>,
}

#[derive(Debug)]
pub struct LinkedList {
    data: Vec<Node>,
    head: Option<usize>,
    empty_head: Option<usize>,
}

pub struct Iter<'a> {
    data: &'a Vec<Node>,
    current_index: Option<usize>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if let Some(index) = self.current_index {
            let item = &self.data[index];
            self.current_index = item.next;
            Some(item.data)
        } else {
            None
        }
    }
}

impl LinkedList {
    pub fn new(size: usize) -> LinkedList {
        let mut list = LinkedList {
            data: Vec::with_capacity(size),
            head: None,
            empty_head: None,
        };
        // create the empty list
        for n in 0usize..size {
            list.data.push(Node{data: 0, next: Some(n+1)});
        }
        if size > 0 {
            list.empty_head = Some(0);
            list.data[size-1] = Node{data: 0, next: None};
        }
        list
    }

    pub fn iter(&self) -> Iter {
        Iter {
            data: &self.data,
            current_index: self.head,
        }
    }

    pub fn push(&mut self, data: i32) {
        let slot = self.get_empty().expect("No more slots");
        if let Some(head) = self.head {
            self.head = Some(slot);
            self.data[slot].data = data;
            self.data[slot].next = Some(head);
        } else {
            self.head = Some(slot);
            self.data[slot] = Node{data: data, next: None};
        }
    }

    pub fn get_head(&self) -> Option<&Node> {
        self.head.map(|h| &self.data[h])
    }

    fn get_empty(&mut self) -> Option<usize> {
        if let Some(empty) = self.empty_head {
            self.empty_head = self.data[empty].next;
            Some(empty)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a LinkedList {
    type Item = i32;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_list() {
        let list = LinkedList::new(0);
        assert_eq!(None, list.get_head());
    }

    #[test]
    fn create_list() {
        let list = LinkedList::new(10);
        assert_eq!(None, list.get_head());
    }

    #[test]
    fn create_list_push() {
        let mut list = LinkedList::new(10);
        list.push(10);
        let head = list.get_head().unwrap();
        assert_eq!(None, head.next);
        assert_eq!(10, head.data);
    }

    #[test]
    fn create_list_push_multiple() {
        let mut list = LinkedList::new(10);
        list.push(10);
        list.push(20);
        let head = list.get_head().unwrap();
        assert_eq!(20, head.data);
        assert!(head.next.is_some());
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::new(10);
        list.push(10);
        list.push(20);

        let copy: Vec<_> = list.iter().collect();

        assert_eq!(vec![20,10], copy);
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new(10);
        list.push(10);
        list.push(10);

        for elem in &list {
            assert_eq!(10, elem);
        }
    }
}
