use std::{marker::PhantomData, mem, ptr::NonNull};
mod pre_implemented;
pub struct Node<T> {
    data: T,
    pub next: Option<NonNull<Node<T>>>,
    pub prev: Option<NonNull<Node<T>>>,
}
pub struct LinkedList<T> {
    pub head: Option<NonNull<Node<T>>>,
    pub tail: Option<NonNull<Node<T>>>,
}
pub struct Cursor<'a, T: 'a> {
    linked_list: &'a mut LinkedList<T>,
    cur: Option<NonNull<Node<T>>>,
}
pub struct Iter<'a, T> {
    _marker: PhantomData<&'a T>,
    cur: Option<NonNull<Node<T>>>,
}
unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}
impl<'a, T> LinkedList<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn len(&self) -> usize {
        self.iter().count()
    }
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            cur: self.head,
            linked_list: self,
        }
    }
    pub fn cursor_back(&'a mut self) -> Cursor<'a, T> {
        Cursor {
            cur: self.tail,
            linked_list: self,
        }
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            _marker: PhantomData,
            cur: self.head,
        }
    }
}
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while cursor.take().is_some() {}
    }
}
impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        Some(&mut unsafe { self.cur?.as_mut() }.data)
    }
    fn next_node(&self) -> Option<NonNull<Node<T>>> {
        unsafe { self.cur?.as_ref() }.next
    }
    fn prev_node(&self) -> Option<NonNull<Node<T>>> {
        unsafe { self.cur?.as_ref() }.prev
    }
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        self.cur = self.next_node();
        self.peek_mut()
    }
    pub fn prev(&mut self) -> Option<&mut T> {
        self.cur = self.prev_node();
        self.peek_mut()
    }
    pub fn take(&mut self) -> Option<T> {
        let cur_node = self.cur?;
        unsafe {
            let prev = cur_node.as_ref().prev;
            let next = cur_node.as_ref().next;
            match next {
                Some(mut n) => n.as_mut().prev = prev,
                None => self.linked_list.tail = prev,
            }
            match prev {
                Some(mut p) => p.as_mut().next = next,
                None => self.linked_list.head = next,
            }
            self.cur = next.or(prev);
            Some(Box::from_raw(cur_node.as_ptr()).data)
        }
    }
    pub fn insert_after(&mut self, element: T) {
        let new = Box::leak(Box::new(Node {
            data: element,
            next: self.next_node(),
            prev: self.cur,
        }));
        let new_node = NonNull::new(new);
        if let Some(mut cur) = self.cur {
            unsafe {
                if let Some(mut n) = mem::replace(&mut cur.as_mut().next, new_node) {
                    n.as_mut().prev = new_node;
                } else {
                    self.linked_list.tail = new_node;
                }
            }
        } else {
            self.linked_list.head = new_node;
            self.linked_list.tail = new_node;
            self.cur = new_node;
        }
    }
    pub fn insert_before(&mut self, element: T) {
        let new = Box::leak(Box::new(Node {
            data: element,
            prev: self.prev_node(),
            next: self.cur,
        }));
        let new_node = NonNull::new(new);
        if let Some(mut cur) = self.cur {
            unsafe {
                if let Some(mut p) = mem::replace(&mut cur.as_mut().prev, new_node) {
                    p.as_mut().next = new_node;
                } else {
                    self.linked_list.head = new_node;
                }
            }
        } else {
            self.linked_list.head = new_node;
            self.linked_list.tail = new_node;
            self.cur = new_node;
        }
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let cur_node = unsafe { self.cur?.as_ref() };
        self.cur = cur_node.next;
        Some(&cur_node.data)
    }
}
