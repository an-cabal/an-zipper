use std::iter;
use super::{List, Node, Stack};

#[cfg(test)] mod test;

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|head| &**head)
             , len: self.len }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|head| &mut **head)
                    , len: self.len }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct Iter<'a, T: 'a>{ next: Option<&'a Node<T>>
                              , len: usize }

impl<'a, T> Iterator for Iter<'a, T>
where T: 'a {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref()
                         .map(|next| &**next);
            self.len -= 1;
            &node.elem
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}


impl<'a, T> iter::ExactSizeIterator for Iter<'a, T> {
    #[inline] fn len(&self) -> usize { self.len }
}

pub struct IterMut<'a, T: 'a>{ next: Option<&'a mut Node<T>>
                                 , len: usize }

impl<'a, T> Iterator for IterMut<'a, T>
where T: 'a {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut()
                         .map(|next| &mut **next);
            self.len -= 1;
            &mut node.elem
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> iter::ExactSizeIterator for IterMut<'a, T> {
    #[inline] fn len(&self) -> usize { self.len }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    #[inline] fn next(&mut self) -> Option<Self::Item> { self.0.pop() }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len, Some(self.0.len))
    }
}

impl<T> iter::ExactSizeIterator for IntoIter<T> {
    #[inline] fn len(&self) -> usize { self.0.len }
}
