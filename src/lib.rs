//! A list with a zipper.
//!
//! A [zipper list] is a linked list structure with a moving cursor. It
//! provides O(1) access to the elements to the immediate left and right of the
//! zipper, and access to arbitrary elements in time proportional to their
//! distance from the zipper.
//!
//! Our zipper list is implemented with a pair of singly-linked lists, so this
//! crate also contains a [singly-linked list implementation].
//!
//! [zipper list]: https://en.wikipedia.org/wiki/Zipper_(data_structure)
//! [singly-linked list implementation]: struct.List.html
#![cfg_attr( feature = "clippy", feature(plugin) )]
#![cfg_attr( feature = "clippy", plugin(clippy) )]

use std::fmt;
use std::iter;

#[macro_use] extern crate unstable_macros;
#[cfg(test)] #[macro_use] extern crate quickcheck;

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
pub mod list;
/// A simple singly-linked list
#[derive(Clone)]
pub struct List<T> { head: Link<T>
                   , len: usize
                   }

type Link<T> = Option<Box<Node<T>>>;


#[derive(Clone)]
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
    #[inline] pub fn len(&self) -> usize { self.len }

    #[inline] pub fn is_empty(&self) -> bool { self.head.is_none() }

    unstable_const_fn! {
        pub const fn new() -> Self {
            List { head: None
                 , len: 0 }
        }
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
}

impl<'a, T> IntoIterator for &'a List<T> {
    type IntoIter = list::Iter<'a, T>;
    type Item = &'a T;

    #[inline] fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type IntoIter = list::IterMut<'a, T>;
    type Item = &'a mut T;

    #[inline] fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = list::IntoIter<T>;

    #[inline] fn into_iter(self) -> Self::IntoIter { self.into_iter() }

}

// impl<T, I> convert::From<I> for List<T>
// where I: IntoIterator<Item=T> {
//     #[inline] fn from(i: I) -> Self { i.into_iter().collect() }
// }

impl<T> iter::FromIterator<T> for List<T> {
    fn from_iter<I>(iter: I) -> Self
    where I: IntoIterator<Item=T> {
        let mut list = List::new();
        for i in iter { list.push(i); }
        list
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

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        for _ in self { }
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
