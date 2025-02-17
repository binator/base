#![doc = include_str!("../readme.md")]
#![cfg_attr(not(test), no_std)]
#![feature(trait_alias)]
#![warn(missing_docs)]
#![deny(clippy::default_numeric_fallback)]

use core::fmt::{
  Debug,
  Display,
  Formatter,
};

mod ascii;
pub use ascii::*;

mod tag;
pub use tag::*;
mod list;
pub use list::*;

mod is;
pub use is::*;

mod one_of;
pub use one_of::*;

mod all;
pub use all::*;
mod end_of_stream;
pub use end_of_stream::*;

// mod character;
// pub use character::*;
mod octet;
pub use octet::*;
mod utf8;
pub use utf8::*;

mod success;
pub use success::*;
mod failure;
pub use failure::*;

mod nbit;
pub use nbit::*;

mod parse;
pub use parse::*;

mod take;
pub use take::*;

mod any;
pub use any::*;

// T sux
/// Atom for base combinator
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseAtom<T: 'static> {
  /// Atom of is combinator
  Is {
    /// Token found if stream is not empty
    t: Option<T>,
    /// Expected Token
    expect: T,
  },
  /// Atom of is_not combinator
  IsNot {
    /// Token found if stream is not empty
    t: Option<T>,
    /// Both Token found and Token that was not expected
    not_expect: T,
  },
  /// Atom of none_of combinator
  NoneOf {
    /// list of not expected Token
    not_expected: &'static [T],
    /// Token found
    found: Option<T>,
  },
  /// Atom of one_of combinator
  OneOf {
    /// List of expected token
    expected: &'static [T],
    /// Token found
    found: Option<T>,
  },
  /// Atom of tag combinator
  Tag {
    /// Expected tag
    tag: &'static str,
  },
  /// Atom of tag_no_case combinator
  TagNoCase {
    /// Expected tag
    tag: &'static str,
  },
  /// Atom of list combinator
  List {
    /// Expected sequence
    list: &'static [T],
  },
  /// Atom of Ascii combinator
  Ascii {
    /// Token found
    found: u8,
    // fixme meh
    /// Expected Token
    expected: &'static str,
  },
  /// Atom of utf8 combinator
  // TODO improve ?
  Utf8 {},
}

impl<T: Display + Debug> Display for BaseAtom<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match self {
      BaseAtom::Is { t, expect } => write!(f, "Is: {:?} != {}", t, expect),
      BaseAtom::IsNot { t, not_expect } => write!(f, "IsNot: {:?} != {}", t, not_expect),
      BaseAtom::NoneOf {
        not_expected,
        found,
      } => write!(f, "NoneOf: not expect {:?} found {:?}", not_expected, found),
      BaseAtom::OneOf { expected, found } => {
        write!(f, "OneOf: expect {:?} found {:?}", expected, found)
      }
      BaseAtom::Tag { tag } => write!(f, "Tag: {:?}", tag),
      BaseAtom::TagNoCase { tag } => {
        write!(f, "TagNoCase: {:?}", tag)
      }
      BaseAtom::List { list } => write!(f, "List: {:?}", list),
      BaseAtom::Ascii { found, expected } => {
        write!(f, "DigitAtom: Found {} Expected {}", found, expected)
      }
      BaseAtom::Utf8 {} => {
        write!(f, "Utf8")
      }
    }
  }
}
