//! Provides the `Single` trait for extracting the element from a
//! single-element iterator.
//!
//! # Examples
//!
//! ```
//! use single::{ Single, Error };
//! use std::iter;
//!
//! assert_eq!(iter::empty::<i32>().single(), Err(Error::NoElements));
//! assert_eq!(iter::once(0).single(), Ok(0));
//! assert_eq!(iter::repeat(0).single(), Err(Error::MultipleElements));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]

#[macro_use]
extern crate failure;
#[cfg(not(feature = "std"))]
use core as std;

#[allow(missing_docs)]
pub trait Single: Iterator {
    /// Get the single element from a single-element iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use single::{ Single, Error };
    /// # use std::iter;
    /// assert_eq!(iter::empty::<i32>().single(), Err(Error::NoElements));
    /// assert_eq!(iter::once(0).single(), Ok(0));
    /// assert_eq!(iter::repeat(0).single(), Err(Error::MultipleElements));
    /// ```
    fn single(self) -> Result<Self::Item, Error>;
}

/// An error in the execution of [`Single::single`](trait.Single.html#tymethod.single).
#[derive(Copy, Clone, Eq, PartialEq, Fail)]
pub enum Error {
    /// Asked an empty iterator for the single element.
    #[fail(display = "Called single() on empty iterator")]
    NoElements,

    /// Asked an iterator with multiple elements for the single element.
    #[fail(display = "Called single() on multiple-element iterator")]
    MultipleElements,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<I: Iterator> Single for I {
    fn single(mut self) -> Result<Self::Item, Error> {
        match self.next() {
            None => Err(Error::NoElements),
            Some(element) => match self.next() {
                None => Ok(element),
                Some(_) => Err(Error::MultipleElements),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::Single;
    use std::iter;

    #[test]
    #[should_panic(expected = "Called single() on empty iterator")]
    fn panic_empty() {
        let _: i32 = iter::empty().single().unwrap();
    }

    #[test]
    #[should_panic(expected = "Called single() on multiple-element iterator")]
    fn panic_multiple() {
        let _ = iter::repeat(0).single().unwrap();
    }
}
