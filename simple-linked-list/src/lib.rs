use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SimpleLinkedList<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0 }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        // Option::take: Takes the value out of the option, leaving a None in its place.
        let node = Box::new(Node::new(element, self.head.take()));
        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        // Option::take: Takes the value out of the option, leaving a None in its place.
        if let Some(h) = self.head.take() {
            self.head = h.next;
            self.len -= 1;
            Some(h.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        // Borrow the head node
        self.head.as_ref().map(|h| &h.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::<T>::new();
        let mut curr = self.head;

        while let Some(c) = curr {
            reversed.push(c.data);
            curr = c.next;
        }
        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::<T>::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut list = Vec::<T>::new();
        let mut reversed = linked_list.rev();
        while let Some(data) = reversed.pop() {
            list.push(data);
        }
        list
    }
}
