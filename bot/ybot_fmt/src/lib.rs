use std::{
    io::{Read, Write, Seek, SeekFrom, Result},
    marker::PhantomData,
    mem::{self, MaybeUninit}
};

pub use action::*;
mod action;

pub const MAGIC: [u8; 4] = *b"ybot";
const HEADER_LEN: u32 = 16_u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PlayerButton {
    Jump = 1,
    Left = 2,
    Right = 3,
}

pub trait Getter {
    type Output;
    fn get<T: Read + Seek>(self,m: &mut Macro<T>) -> Result<Self::Output>;
}

pub trait Setter {
    type Input;

    fn set<T: Read + Write + Seek>(self, m: &mut Macro<T>, value: Self::Input) -> Result<()>;
}

#[derive(Clone, Copy)]
pub struct Meta<T> {
    offset: u32,
    _ph: PhantomData<T>,
}

pub struct Macro<T> {
    inner: T,
    version: u32,
    meta_len: u32,
    blobs: u32,
    acts_start: u32,
}

macro_rules! def_meta {
	(@inner [$offset:expr]) => {};
	(@inner [$offset:expr] $(#[$meta:meta])* $name:ident: $ty:ty, $($rest:tt)*) => {
		impl Meta<$ty> {
			$(#[$meta])*
			pub const $name: Self = Self::new($offset as u32);
		}
		def_meta!(@inner [$offset + mem::size_of::<$ty>()] $($rest)*);
	};
	($($tt:tt)*) => { def_meta!(@inner [0] $($tt)*); };
}

def_meta! {
    /// The UNIX timestamp of when the macro was created.
    DATE: i64,
    /// The amount of presses (i.e. clicks, or left/right inputs in platformer mode) in the level.
    PRESSES: u64,
    /// The amount of frames in the level (one more than the frame of the last action).
    FRAMES: u64,
    /// The FPS of the macro.
    FPS: f32,
    /// The total amount of presses while botting the level.
    TOTAL_PRESSES: u64,
}