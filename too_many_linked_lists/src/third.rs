use std::rc::Rc;

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn prepend(&self, elem: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                elem: elem, next: self.head.clone()
            }))
        }
    }

    pub fn tail(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|rc_node| {
                rc_node.next.clone()
            })
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|rc_node| {
            &rc_node.elem
        })
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(rc_node) = head {
            if let Ok(node) = Rc::try_unwrap(rc_node) {
                head = node.next;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        
    }
}
