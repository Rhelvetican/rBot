use std::{
    io::{Read, Write, Seek, SeekFrom, Result},
    marker::PhantomData,
    mem::{self, MaybeUninit}
};

pub use action::*;
mod action;

pub const MAGIC: [u8; 4] = *b"ybot";