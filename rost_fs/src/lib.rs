#![feature(alloc, try_trait, ptr_internals)]
#![feature(test)]
#![allow(unused)]
#![no_std]

#[macro_use]
extern crate alloc;

pub mod node;
pub mod disk;
pub mod fs;


#[cfg(test)]
mod tests;

