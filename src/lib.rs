pub use crate::keypad::KeypadKey;
pub use crate::gpu::{SCREEN_W, SCREEN_H};
pub use crate::sound::{AudioPlayer, Sound};

pub mod cpu;

mod gbmode;
mod gpu;
mod keypad;
mod mbc3;
mod mmu;
mod register;
mod rom;
mod serial;
mod sound;
mod timer;

pub type StrResult<T> = Result<T, &'static str>;