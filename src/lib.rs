//!
//! Assert is used to create generic trait bounds:
//!
//! ```
//! #![allow(incomplete_features)]
//! #![feature(generic_const_exprs)]
//!
//! use const_assert::{Assert, IsTrue, IsFalse};
//!
//! struct Buffer<const N: usize> {
//!   inner: [usize; N],
//! }
//!
//! impl<const N: usize> Buffer<N>
//! where
//!   Assert<{ N == N.next_power_of_two() }>: IsTrue,
//!   Assert<{ N == 1 }>: IsFalse
//! {
//!   pub const fn new() -> Self {
//!       Buffer { inner: [0; N] }
//!   }
//! }
//!
//! static BUFFER: Buffer<1024> = Buffer::new();
//! ```

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#![no_std]

pub struct Assert<const COND: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

pub trait IsFalse {}

impl IsFalse for Assert<false> {}
