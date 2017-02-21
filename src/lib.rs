#[derive(Debug, PartialEq, Eq)]
pub enum Node<T> {
    Empty{next: Option<usize>},
    Used{data: T, next: Option<usize>},
}

#[derive(Debug)]
pub struct LinkedList<T> {
    data: Vec<Node<T>>,
    head: Option<usize>,
    empty_head: Option<usize>,
}

pub struct Iter<'a, T:'a> {
    data: &'a Vec<Node<T>>,
    current_index: Option<usize>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(index) = self.current_index {
            let item = &self.data[index];
            //self.current_index = item.next;
            //Some(item.data)
            None
        } else {
            None
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new(size: usize) -> LinkedList<T> {
        let mut list = LinkedList::<T> {
            data: Vec::with_capacity(size),
            head: None,
            empty_head: None,
        };
        // create the empty list
        for n in 0usize..size {
            //list.data.push(Node::<T>::Empty{next: Some(n+1)});
        }
        if size > 0 {
            list.empty_head = Some(0);
            //list.data[size-1] = Node::<T>Empty{data: 0, next: None};
        }
        list
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            data: &self.data,
            current_index: self.head,
        }
    }

    pub fn push(&mut self, data: i32) {
        let slot = self.get_empty().expect("No more slots");
        if let Some(head) = self.head {
            self.head = Some(slot);
            //self.data[slot].data = data;
            //self.data[slot].next = Some(head);
        } else {
            self.head = Some(slot);
            //self.data[slot] = Node{data: data, next: None};
        }
    }

    pub fn get_head(&self) -> Option<&Node<T>> {
        self.head.map(|h| &self.data[h])
    }

    fn get_empty(&mut self) -> Option<usize> {
        if let Some(empty) = self.empty_head {
            //self.empty_head = self.data[empty].next;
            Some(empty)
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_list() {
        let list = LinkedList::<i32>::new(0);
        assert_eq!(None, list.get_head());
    }

    #[test]
    fn create_list() {
        let list = LinkedList::<i32>::new(10);
        assert_eq!(None, list.get_head());
    }

    #[test]
    fn create_list_push() {
        let mut list = LinkedList::new(10);
        list.push(10);
        let head = list.get_head().unwrap();
        match head {
            &Node::Used{data, next} => {
                assert_eq!(None, next);
                assert_eq!(10, data);
            },
            _ => panic!(""),
        }
    }

    #[test]
    fn create_list_push_multiple() {
        let mut list = LinkedList::new(10);
        list.push(10);
        list.push(20);
        let head = list.get_head().unwrap();
        match head {
            &Node::Used{data, next} => {
                assert!(next.is_some());
                assert_eq!(20, data);
            },
            _ => panic!(""),
        }
        //assert_eq!(20, head.data);
        //assert!(head.next.is_some());
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
