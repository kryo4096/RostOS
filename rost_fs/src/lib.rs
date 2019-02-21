#![feature(alloc, try_trait, ptr_internals)]
#![feature(test)]
#![allow(unused)]
#![no_std]

#[macro_use]
extern crate alloc;

pub mod disk;
pub mod fs;
pub mod node;

#[cfg(test)]
mod tests;
