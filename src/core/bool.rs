#![allow(dead_code)]
use std::{
  fmt::{Debug, Display, Formatter},
  fmt,
};
use ::condition::prelude::*;

// with, without
// enable, disable
// on, off
// true, false
// yes, no
// 1, 0
// allow, forbid

pub enum Bool {
  Disable,
  False,
  Off,
  No,
  Enable,
  True,
  Yes,
  On,
}

impl Default for Bool {
  fn default () -> Self { Bool::Off }
}

impl Condition for Bool {
  #[inline]
  fn is (&self, value: bool) -> bool {
    use Bool::*;
    match self {
      Disable | False | Off | No => value == false,
      Enable | True | Yes | On => value == true,
    }
  }
}

impl Debug for Bool {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use Bool::*;
    let text = match self {
      Disable => "disabled",
      Enable => "enabled",
      False => "false",
      True => "true",
      Off => "off",
      On => "on",
      Yes => "yes",
      No => "no",
    };
    Display::fmt(text, f)
  }
}

impl From<Bool> for bool {
  fn from (b: Bool) -> Self {
    use Bool::*;
    match b {
      Disable | False | Off | No => false,
      Enable | True | Yes | On => true,
    }
  }
}

impl Display for Bool {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let text = if self.is_true() { "true" } else { "false" };
    Display::fmt(text, f)
  }
}
