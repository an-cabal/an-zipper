use std::fmt;
use std::iter;

#[macro_use] extern crate unstable_macros;

/// Trait describing stack behaviour
pub trait Stack<T> {
    /// Push `elem` to the stack.
    ///
    /// # Arguments
    /// - `elem`: an item of type `T` to be pushed to the stack
    ///
    /// # Returns
    /// `&mut Self` so that multiple `push`es can be chained.
    fn push(&mut self, elem: T) -> &mut Self;

    /// Remove the top item of the stack if it exists, returning it.
    ///
    /// # Returns
    /// - `Some(T)` if an item was popped
    /// - `None` if the stack is empty
    fn pop(&mut self) -> Option<T>;

    /// Borrow the top item of the stack if it exists.
    ///
    /// # Returns
    /// - `Some(&T)` if an item was popped
    /// - `None` if the stack is empty
    fn peek(&self) -> Option<&T>;

    /// Mutably borrow the top item of the stack if it exists.
    ///
    /// # Returns
    /// - `Some(&mut T)` if an item was popped
    /// - `None` if the stack is empty
    fn peek_mut(&mut self) -> Option<&mut T>;
}

//==- singly-linked list -===================================================
/// A simple singly-linked list
pub struct List<T> { head: Link<T>
                   , len: usize
                   }

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> { elem: T
               , next: Link<T>
               }

impl<T> fmt::Debug for Node<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!( f, "{:?}{}"
              , self.elem
              , self.next.as_ref()
                    .map(|next| format!(", {:?}", next))
                    .unwrap_or_else(|| { String::new() })
              )
    }
}

impl<T> fmt::Display for Node<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!( f, "{}{}"
              , self.elem
              , self.next.as_ref()
                    .map(|next| format!(", {}", next))
                    .unwrap_or_else(|| { String::new() })
              )
    }
}

impl<T> Node<T> {

    unstable_const_fn!{
        pub const fn new(elem: T) -> Self { Node { elem: elem, next: None } }
    }

}

impl<T> Stack<T> for List<T> {
    fn push(&mut self, elem: T) -> &mut Self {
        self.cons(Box::new(Node::new(elem)))
    }

    fn pop(&mut self) -> Option<T> {
        self.uncons().map(|node| node.elem)
    }

    #[inline]
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem )
    }

    #[inline]
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem )
    }
}


impl<T> List<T> {
    /// Returns the length of the list
    ///
    /// # Time complexity
    /// O(1)
    #[inline] fn len(&self) -> usize { self.len }

    #[inline] fn is_empty(&self) -> bool { self.head.is_none() }

    #[inline] fn new() -> Self {
        List { head: None
             , len: 0 }
    }

    fn cons(&mut self, mut node: Box<Node<T>>) -> &mut Self {
        node.next = self.head.take();
        self.head = Some(node);
        self.len += 1;
        self
    }

    fn uncons(&mut self) -> Link<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            self.len -= 1;
            node
        })
    }

    pub fn iter(&self) -> ListIter<T> {
        ListIter(self.head.as_ref().map(|head| &**head))
    }

    pub fn iter_mut(&mut self) -> ListIterMut<T> {
        ListIterMut(self.head.as_mut().map(|head| &mut **head))
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type IntoIter = ListIter<'a, T>;
    type Item = &'a T;

    #[inline] fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type IntoIter = ListIterMut<'a, T>;
    type Item = &'a mut T;

    #[inline] fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> iter::Extend<T> for List<T>  {
    fn extend<I>(&mut self, iter: I)
    where I: IntoIterator<Item=T> {
        for i in iter { self.push(i); }
    }
}

impl<'a, T> iter::Extend<&'a T> for List<T>
where T: Copy + 'a {

    fn extend<I>(&mut self, iter: I)
    where I: IntoIterator<Item=&'a T> {
        for i in iter { self.push(*i); }
    }
}

impl<T> fmt::Debug for List<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!( f, "[{:?}]"
              , self.head.as_ref()
                    .map(|head| format!("{:?}", head))
                    .unwrap_or_else(|| { String::new() })
              )
    }
}

impl<T> fmt::Display for List<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!( f, "[{}]"
              , self.head.as_ref()
                    .map(|head| format!("{}", head))
                    .unwrap_or_else(|| { String::new() })
              )
    }
}

pub struct ListIter<'a, T: 'a>(Option<&'a Node<T>>);

impl<'a, T> Iterator for ListIter<'a, T>
where T: 'a {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = node.next.as_ref()
                         .map(|next| &**next);
            &node.elem
        })
    }
}

pub struct ListIterMut<'a, T: 'a>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for ListIterMut<'a, T>
where T: 'a {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_mut()
                         .map(|next| &mut **next);
            &mut node.elem
        })
    }
}

//==- zip list -=============================================================
/// A linked list with a zipper
pub struct ZipList<T> { left: List<T>
                      , right: List<T>
                      }

impl<T> ZipList<T> {

    unstable_const_fn!{
        pub const fn new() -> Self {
            ZipList { left: List::new(), right: List::new() }
        }
    }

    // -- wrappers around sublist methods -----------------------------------
    #[inline] pub fn pop_left(&mut self) -> Option<T> { self.left.pop() }
    #[inline] pub fn pop_right(&mut self) -> Option<T> { self.right.pop() }
    #[inline] pub fn peek_left(&self) -> Option<&T> { self.left.peek() }
    #[inline] pub fn peek_right(&self) -> Option<&T> { self.right.peek() }
    #[inline] pub fn peek_left_mut(&mut self) -> Option<&mut T> {
        self.left.peek_mut()
     }
    #[inline] pub fn peek_right_mut(&mut self) -> Option<&mut T> {
        self.right.peek_mut()
    }

    #[inline] pub fn push_left(&mut self, elem: T) -> &mut Self {
         self.left.push(elem);
         self
     }

    #[inline] pub fn push_right(&mut self, elem: T) -> &mut Self {
        self.left.push(elem);
        self
    }

    #[inline] pub fn len(&self) -> usize { self.left.len() + self.right.len() }

    #[inline] pub fn is_empty(&self) -> bool {
        self.left.is_empty() && self.right.is_empty()
    }

    pub fn move_left(&mut self) -> bool {
        self.left.uncons()
            .map(|n| self.right.cons(n))
            .is_some()
    }

    pub fn move_right(&mut self) -> bool {
        self.right.uncons()
            .map(|n| self.left.cons(n))
            .is_some()
    }

    pub fn seek_left(&mut self, amount: usize) {
        unimplemented!()
    }

    pub fn seek_right(&mut self, amount: usize) {
        unimplemented!()
    }

}


impl<T> fmt::Debug for ZipList<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!( f, "[{:?}_{:?}]"
              , self.left.head.as_ref()
                    .map(|head| format!("{:?}, ", head))
                    .unwrap_or_else(String::new)
              , self.right.head.as_ref()
                    .map(|head| format!(", {:?}", head))
                    .unwrap_or_else(String::new)
              )
    }
}

impl<T> fmt::Display for ZipList<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!( f, "[{}_{}]"
              , self.left.head.as_ref()
                    .map(|head| format!("{}, ", head))
                    .unwrap_or_else(|| { String::new() })
              , self.right.head.as_ref()
                    .map(|head| format!(", {}", head))
                    .unwrap_or_else(String::new)
              )
    }
}
