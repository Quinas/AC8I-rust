extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

mod chip;
mod instruction;
pub mod system;

pub trait ChipSound {
  fn make_sound(&self);
}
